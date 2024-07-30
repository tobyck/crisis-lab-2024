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

use crate::config::SENSOR_HEIGHT_FROM_FLOOR;

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
            // otherwise we can overwrite whatever is there already
            self.content[self.next_index] = value;
        }

        self.next_index += 1;
        self.next_index %= self.capacity;
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn to_vec(&self) -> Vec<T> {
        // because the ring buffer can start at any point in the vec, to convert it to a vec in the
        // right order it needs to have two pieces stitched together. this joins from the start of
        // the buffer to then end of the vec, and the beginning of the vec to right before the
        // start.
        self.content[self.next_index..]
            .iter()
            .chain(self.content[..self.next_index].iter())
            .copied()
            .collect()
    }

    pub fn last(&self) -> Option<T> {
        if self.content.len() > 0 {
            if self.next_index == 0 {
                // if the next index is 0 then we're about to wrap around but we haven't yet, so
                // the last item in the cache is the last item in the vec
                self.content.last().copied()
            } else {
                Some(self.content[self.next_index - 1])
            }
        } else {
            None
        }
    }

    pub fn at(&self, index: usize) -> Option<T> {
        if index > self.content.len() - 1 {
            return None;
        }

        Some(self.content[(self.next_index + index) % self.content.len()])
    }

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
            Some(Box::new(self.content[self.next_index - n..self.next_index].iter()))
        } else {
            // otherwise, we need to wrap around and concatenate two sections
            Some(Box::new(
                // n - self.next_index is the amount that we need from the end of the vec
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

    #[serde(with = "serde_millis")] // this is needed for the Instant to serialise with Serde
    timestamp: Instant
}

impl DataPacket {
    // returns a data packet with no computed height, only raw pressure
    pub fn unprocessed(pressure: f32) -> Self {
        Self {
            pressure,
            height: None,
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

// this is the data sent when a client connects for the first time
#[derive(Serialize)]
pub struct InitialDataPacket {
    pub previous_data: Vec<DataPacket>,
    pub previous_alerts: Vec<Alert>,
    pub calibrations: Calibrations,
    pub alert_threshold: f32
}

pub fn height_from_pressure(pressure: f32, air_pressure: f32) -> f32 {
    // these need to be f32s because the values in the cache are f32s to save space
    const WATER_DENSITY: f32 = 998.0;
    const AIR_DENSITY: f32 = 1.2;
    const GRAVITY: f32 = 9.80248; // got this from latitude and altitude standing outside massey

    // The formula is height (m) = change in pressure (Pa) / change in density (kg/m^3) / acceleration
    // due to gravity (m/s^2). But our input is in hPa, so multiply by 100 to get Pa, and output is in
    // metres, so we multiply by 100 again to get cm.
    ((pressure - air_pressure) * 100.0) / (WATER_DENSITY - AIR_DENSITY) / GRAVITY * 100.0 + SENSOR_HEIGHT_FROM_FLOOR
}

pub async fn process_data(
    water_pressure: f32,
    air_pressure: f32
) -> DataPacket {
    let height_from_floor = height_from_pressure(water_pressure, air_pressure);

    let data = DataPacket {
        pressure: water_pressure,
        height: Some(height_from_floor),
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
        let mut cache = Cache::new(4);
        cache.write(8);
        assert!(cache.content.eq(&vec![8]));

        let mut cache = Cache::new(4);
        cache.write(8);
        cache.write(1);
        assert!(cache.content.eq(&vec![8, 1]));

        let mut cache = Cache::new(4);
        cache.write(8);
        cache.write(1);
        cache.write(3);
        cache.write(6);
        cache.write(4);
        assert!(cache.content.eq(&vec![4, 1, 3, 6]));

        let mut cache = Cache::new(4);
        cache.write(8);
        cache.write(1);
        cache.write(3);
        cache.write(6);
        assert!(cache.content.eq(&vec![8, 1, 3, 6]));

        let mut cache = Cache::new(4);
        cache.write(6);
        cache.write(5);
        cache.write(4);
        cache.write(3);
        cache.write(2);
        cache.write(1);
        assert!(cache.content.eq(&vec![2, 1, 4, 3]));
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
    fn len() {
        let cache = Cache::<i32>::new(4);
        assert_eq!(cache.len(), 0);

        let cache = cache_from_iter(4, [1, 2]);
        assert_eq!(cache.len(), 2);

        let cache = cache_from_iter(4, [1, 2, 3, 4, 5]);
        assert_eq!(cache.len(), 4);
    }

    #[test]
    fn to_vec() {
        let cache = Cache::<i32>::new(4);
        assert_eq!(cache.to_vec(), Vec::<i32>::new());

        let cache = cache_from_iter(4, [1, 2, 3]);
        assert_eq!(cache.to_vec(), vec![1, 2, 3]);

        let cache = cache_from_iter(4, [1, 2, 3, 4, 5, 6]);
        assert_eq!(cache.to_vec(), vec![3, 4, 5, 6]);
    }

    #[test]
    fn last() {
        let cache = cache_from_iter(4, [1, 2]);
        assert_eq!(cache.last(), Some(2));

        let cache = cache_from_iter(4, [0, 1, 2, 3, 4]);
        assert_eq!(cache.last(), Some(4));

        let cache = Cache::<i32>::new(0);
        assert_eq!(cache.last(), None);

        let cache = cache_from_iter(4, [1, 2, 3, 4]);
        assert_eq!(cache.last(), Some(4));
    }

    #[test]
    fn at() {
        let cache = cache_from_iter(4, [5, 4, 7, 2]);
        assert_eq!(cache.at(1), Some(4));

        let cache = cache_from_iter(4, [1, 0, 9, 5, 0, 4]);
        assert_eq!(cache.at(2), Some(0));

        let cache = cache_from_iter(4, [3, 6, 8, 5]);
        assert_eq!(cache.at(0), Some(3));

        let cache = cache_from_iter(4, [5, 4, 7, 2]);
        assert_eq!(cache.at(3), Some(2));

        let cache = cache_from_iter(4, [5, 4, 7, 2]);
        assert_eq!(cache.at(4), None);

        let cache = cache_from_iter(4, [1, 0, 9, 5, 0, 4]);
        assert_eq!(cache.at(4), None);
    }

    #[test]
    fn last_n() {
        let cache = cache_from_iter(4, [1, 2, 3]);
        itertools::assert_equal(cache.last_n(2).unwrap(), [2, 3].iter());

        let cache = cache_from_iter(4, [1, 2, 3, 4, 5, 6]);
        itertools::assert_equal(cache.last_n(1).unwrap(), [6].iter());

        let cache = cache_from_iter(4, [1, 2, 3]);
        assert!(cache.last_n(4).is_none());

        let cache = Cache::<i32>::new(4);
        itertools::assert_equal(cache.last_n(0).unwrap(), std::iter::empty::<&i32>());

        let cache = cache_from_iter(4, [1, 2, 3, 4]);
        itertools::assert_equal(cache.last_n(2).unwrap(), [3, 4].iter());

        let cache = cache_from_iter(4, [1, 2, 3, 4, 5, 6]);
        itertools::assert_equal(cache.last_n(3).unwrap(), [4, 5, 6].iter());
    }
}
