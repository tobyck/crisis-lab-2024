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
use log::debug;

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

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.content[self.next_index..]
            .iter()
            .chain(self.content[..self.next_index].iter())
            .copied()
            .collect()
    }

    pub fn last(&self) -> Option<T> {
        if self.content.len() > 0 {
            if self.next_index == 0 {
                self.content.last().copied()
            } else {
                Some(self.content[self.next_index - 1])
            }
        } else {
            None
        }
    }

    pub fn at(&self, index: usize) -> Option<T> {
        if index < self.content.len() {
            if self.content.len() < self.capacity {
                Some(self.content[index])
            } else {
                Some(self.content[(self.next_index + index) % self.content.len()])
            }
        } else {
            None
        }
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

    #[serde(with = "serde_millis")]
    timestamp: Instant
}

impl DataPacket {
    pub fn unprocessed(pressure: f32) -> Self {
        Self {
            pressure,
            height: None,
            waveform: None,
            timestamp: Instant::now()
        }
    }

    // allow read access to some properties
    pub fn get_pressure(&self) -> f32 { self.pressure }
    pub fn get_height(&self) -> Option<f32> { self.height }
    pub fn get_timestamp(&self) -> Instant { self.timestamp }
}

// cache of processed data wrapped in Arc and RwLock to make it thread-safe
pub type SharedCache = Arc<RwLock<Cache<DataPacket>>>;

#[derive(Clone, Serialize)]
pub struct Alert {
    pub height: f32,

    #[serde(with = "serde_millis")]
    pub timestamp: Instant
}

pub type SharedAlertsVec = Arc<RwLock<Vec<Alert>>>;

// wrapping these two values in a struct to make them easier to pass around
#[derive(Serialize, Clone)]
pub struct Calibrations {
    pub air_pressure: Option<f32>,
    pub resting_water_level: Option<f32>
}

pub type SharedCalibrations = Arc<RwLock<Calibrations>>;

// this is the data send when a client connects for the first time
#[derive(Serialize)]
pub struct InitialDataPacket {
    pub previous_data: Vec<DataPacket>,
    pub previous_alerts: Vec<Alert>,
    pub calibrations: Calibrations
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
) -> DataPacket {
    let wave_height: f32 = height_from_pressure(water_pressure, air_pressure) - resting_water_level;
    let waveform: f32 = 0.0; // TODO: actually calculate this

    let data = DataPacket {
        pressure: water_pressure,
        height: Some(wave_height),
        waveform: Some(waveform),
        timestamp: Instant::now()
    };

    debug!("Computed data packet: {:?}", data);

    data
}

#[cfg(test)]
mod cache_tests {
    use super::Cache;

    #[test]
    fn write() {
        let mut cache: Cache<i32> = Cache::new(3);

        cache.write(8);
        assert_eq!(cache.content.len(), 1);
        assert!(cache.content.eq(&vec![8]));

        cache.write(1);
        assert_eq!(cache.content.len(), 2);
        assert!(cache.content.eq(&vec![8, 1]));
    }

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
    fn last() {
        let cache = cache_from_iter(3, [1, 4, 9]);
        let last = cache.last();
        assert_eq!(last, Some(9));

        let cache = cache_from_iter(3, [0, 1, 4, 9]);
        let last = cache.last();
        assert_eq!(last, Some(9));

        let cache: Cache<i32> = Cache::new(0);
        let last = cache.last();
        assert_eq!(last, None);
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

        let cache: Cache<i32> = Cache::new(0);
        let last2 = cache.last_n(2);
        assert!(last2.is_none());
    }

    #[test]
    fn at() {
        let cache = cache_from_iter(5, [5, 4, 7]);
        assert_eq!(cache.at(0), Some(5));
        assert_eq!(cache.at(1), Some(4));
        assert_eq!(cache.at(2), Some(7));
        assert_eq!(cache.at(3), None);

        let cache = cache_from_iter(3, [0, 9, 2, 1]);
        assert_eq!(cache.at(0), Some(9));
        assert_eq!(cache.at(1), Some(2));
        assert_eq!(cache.at(2), Some(1));
        assert_eq!(cache.at(3), None);
    }
}
