/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* 
* This file contains all the code related to the data processing. This
* includes: the function to process raw pressure data from the sensor, a struct 
* to hold the processed data, and a cache for storing recent data.
*/

use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Cache<T> {
    content: Vec<T>,
    capacity: usize,
    next_index: usize
}

impl<T> Cache<T> {
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
}

// cache of processed data wrapped in Arc and RwLock to make it thread-safe
pub type SharedCache = Arc<RwLock<Cache<ProcessedData>>>;

#[derive(Debug)]
pub struct ProcessedData {
    pub pressure: f32,
    pub wave_height: f32
}

pub fn process_and_cache_data(pressure: f32, cache: &SharedCache) {
    let mut lock = cache.write().unwrap();

    lock.write(ProcessedData {
        pressure,
        wave_height: 0.0
    });
}
