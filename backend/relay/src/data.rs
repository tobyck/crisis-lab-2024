/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* 
* This file contains all the code related to the data processing. This
* includes: the function to process raw pressure data from the sensor, a struct 
* to hold the processed data, and a cache for storing recent data.
* */

use std::{env, sync::Arc, time::Instant};

use serde_json::json;
use tokio::sync::RwLock;
use serde::Serialize;
use log::{error, info};

use crate::config::{ALERT_COOLDOWN, ALERT_ENDPOINT, ALERT_THRESHOLD};

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
        self.content[self.next_index..]
            .iter()
            .chain(self.content[..self.next_index].iter())
            .copied()
            .collect()
    }

    // returns None if not enough data is in the cache
    pub fn last_n<'a>(&'a self, n: usize) -> Option<Box<dyn Iterator<Item = &'a T> + 'a>> {
        let length = self.content.len();

        if n > length {
            return None;
        }

        if self.next_index == 0 {
            // if the end of the cache is at end of the vec then just get the last n items normally
            Some(Box::new(self.content[length - n..].iter()))
        } else if n <= self.next_index {
            // if the end of the cache is somewhere else but we don't need to wrap around
            let end_index = if self.next_index == 0 { length } else { self.next_index };
            Some(Box::new(self.content[end_index - n..end_index].iter()))
        } else {
            // otherwise, we need to wrap around and concatenate two sections
            Some(Box::new(
                self.content[length - (n - self.next_index)..]
                    .iter()
                    .chain(self.content[..self.next_index].iter())
            ))
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct DataPacket {
    pressure: f32,
    height: Option<f32>,
    waveform: Option<f32>,

    // this says whether or not to trigger a new alert (not whether or not then
    // wave height is above the threshold)
    trigger_alert: bool,

    #[serde(with = "serde_millis")]
    timestamp: Instant
}

impl DataPacket {
    pub fn with_only_pressure(pressure: f32) -> Self {
        Self {
            pressure,
            height: None,
            waveform: None,
            trigger_alert: false,
            timestamp: Instant::now()
        }
    }

    // allow read access to the pressure via this method
    pub fn get_pressure(&self) -> f32 {
        self.pressure
    }
}

// cache of processed data wrapped in Arc and RwLock to make it thread-safe
pub type SharedCache = Arc<RwLock<Cache<DataPacket>>>;

#[derive(Clone, Serialize)]
pub struct Alert {
    wave_height: f32,

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

async fn post_alert(height: f32) {
    let password = env::var("ALERT_PASSWORD").expect("Error reading ALERT_PASSWORD environment variable");

    let body = json!({
        "height": height,
        "password": password
    });

    // send the above json the social alerts system
    let client = reqwest::Client::new();
    let response = client.post(ALERT_ENDPOINT)
        .json(&body)
        .send()
        .await;

    if let Err(error) = response {
        error!("Error trying to post alert to {}: {}", ALERT_ENDPOINT, error);
    }
}

pub fn height_from_pressure(pressure: f32, air_pressure: f32) -> f32 {
    // these need to be f32s because the values in the cache are f32s to save space
    const WATER_DENSITY: f32 = 998.0;
    const AIR_DENSITY: f32 = 1.2;
    const GRAVITY: f32 = 9.8;

    // The formula is height (m) = change in pressure (Pa) / change in density (kg/m^3) / acceleration
    // due to gravity (m/s^2). But our input is in hPa, so multiply by 100 to get Pa, and output is in
    // metres, so we multiply by 100 again to get cm.
    ((pressure - air_pressure) * 100.0) / (WATER_DENSITY - AIR_DENSITY) / GRAVITY * 100.0
}

pub async fn process_data(
    water_pressure: f32,
    air_pressure: f32,
    resting_water_level: f32,
    cache: &SharedCache,
    alerts: &SharedAlertsVec
) -> DataPacket {
    let mut alerts_lock = alerts.write().await;

    let wave_height: f32 = height_from_pressure(water_pressure, air_pressure) - resting_water_level;
    let waveform: f32 = 0.0; // TODO: actually calculate this
    
    let trigger_alert = wave_height >= ALERT_THRESHOLD && match alerts_lock.last() {
        // only make another alert if the previous one was long enough ago
        Some(alert) => alert.timestamp.elapsed() > ALERT_COOLDOWN,
        // if there haven't been any alerts yet then we should certainly alert
        None => true
    };

    if trigger_alert {
        let alert = Alert {
            wave_height,
            timestamp: Instant::now()
        };

        alerts_lock.push(alert);

        let alerts_var = env::var("ALERTS");

        // only alert if ALERTS isn't set or it doesn't equal 0
        if alerts_var.is_err() || alerts_var.is_ok_and(|x| x != "0".to_string()) {
            info!("Posting alert to social alerts system");
            post_alert(wave_height).await;
        }
    }

    let data = DataPacket {
        pressure: water_pressure,
        height: Some(wave_height),
        waveform: Some(waveform),
        trigger_alert,
        timestamp: Instant::now()
    };

    cache.write().await.write(data);

    data
}

// TODO: write tests for everything else
#[cfg(test)]
mod tests {
    use super::Cache;

    fn cache_from_iter<T, I>(capacity: usize, items: I) -> Cache<T>
    where
        T: Copy + Send,
        I: IntoIterator<Item = T>
    {
        let mut cache = Cache::new(capacity);

        for item in items.into_iter() {
            cache.write(item);
        }

        cache
    }

    #[test]
    fn to_vec() {
        let cache = cache_from_iter(3, [1, 2, 3]);
        assert_eq!(cache.to_vec(), vec![1, 2, 3]);

        let cache: Cache<i32> = Cache::new(3);
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(cache.to_vec(), empty_vec);
    }

    #[test]
    fn last_n() {
        let cache = cache_from_iter(5, [1, 4, 8, 5, 6]);
        let last3 = cache.last_n(3);
        assert!(last3.is_some());
        assert!(last3.unwrap().eq([8, 5, 6].iter()));

        let cache = cache_from_iter(5, [0, 0, 8, 5, 6, 1, 4]);
        let last4 = cache.last_n(4);
        assert!(last4.is_some());
        assert!(last4.unwrap().eq([5, 6, 1, 4].iter()));

        let cache = cache_from_iter(5, [0, 0, 0, 0, 6, 1, 4, 8, 5,]);
        let last2 = cache.last_n(2);
        assert!(last2.is_some());
        assert!(last2.unwrap().eq([8, 5].iter()));
    }
}
