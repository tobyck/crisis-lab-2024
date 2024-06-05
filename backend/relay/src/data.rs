/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* 
* This file contains all the code related to the data processing. This
* includes: the function to process raw pressure data from the sensor, a struct 
* to hold the processed data, and a cache for storing recent data.
* */

use std::{sync::Arc, time::Instant};
use tokio::sync::RwLock;

use serde::Serialize;

use crate::config::{ALERT_COOLDOWN, ALERT_THRESHOLD};

#[derive(Debug)]
pub struct Cache<T> {
    content: Vec<T>,
    capacity: usize,
    next_index: usize
}

impl<T: Copy + Send> Cache<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            // the size of the cache is fixed so we can just do one single memory allocation
            content: Vec::with_capacity(capacity),
            capacity,
            next_index: 0
        }
    }

    pub fn write(&mut self, value: T) {
        // if not all of the space allocated has been used then .push is needed to also increase
        // the vector's length
        if self.content.len() < self.capacity {
            self.content.push(value);
        } else {
            // otherwise we can overwrite whatever's there already
            self.content[self.next_index] = value;
        }

        self.next_index += 1;
        self.next_index %= self.capacity;
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.content[self.next_index..].iter()
            .chain(self.content[..self.next_index].iter())
            .copied()
            .collect()
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct DataPacket {
    pressure: f32,
    height: f32,
    waveform: f32,

    // this says whether or not to trigger a new alert (not whether or not then
    // wave height is above the threshold)
    trigger_alert: bool,

    #[serde(with = "serde_millis")]
    timestamp: Instant
}

// cache of processed data wrapped in Arc and RwLock to make it thread-safe
pub type SharedCache = Arc<RwLock<Cache<DataPacket>>>;

#[derive(Clone, Serialize)]
pub struct Alert {
    height: f32,

    #[serde(with = "serde_millis")]
    timestamp: Instant
}

pub type SharedAlertsVec = Arc<RwLock<Vec<Alert>>>;

// this is the data send when a client connects for the first time
#[derive(Serialize)]
pub struct InitialDataPacket {
    pub previous_data: Vec<DataPacket>,
    pub previous_alerts: Vec<Alert>
}

pub async fn process_data(pressure: f32, cache: &SharedCache, alerts: &SharedAlertsVec) -> DataPacket {
    let mut alerts_lock = alerts.write().await;

    // todo: actually calculate these
    let height: f32 = 8.0;
    let waveform: f32 = 0.0;
    
    let trigger_alert = height >= ALERT_THRESHOLD && match alerts_lock.last() {
        // only make another alert if the previous one was long enough ago
        Some(alert) => alert.timestamp.elapsed() > ALERT_COOLDOWN,
        // if there haven't been any alerts yet then we should certainly alert
        None => true
    };

    let data = DataPacket {
        pressure,
        height,
        waveform,
        trigger_alert,
        timestamp: Instant::now()
    };

    if trigger_alert {
        alerts_lock.push(Alert {
            height,
            timestamp: Instant::now()
        });
    }

    cache.write().await.write(data);

    data
}
