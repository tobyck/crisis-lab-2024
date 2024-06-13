/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This file handles everything MQTT related -- intialising the client with the
* appropriate options, polling the event loop for messages, passing the message
* to the function in data.rs to process the data, and sending it to the
* WebSocket handlers.
* */

use std::{env, fs::File, io::Write, sync::Arc, time::Duration};

use log::{debug, error, info, log_enabled, warn};
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet};
use serde_json::json;
use tokio::sync::{broadcast::{self, Sender}, RwLock};

use crate::{
    alert::check_for_alert, config::{CACHE_CAPACITY, CHANNEL_CAPACITY, FREQUENCY, MAX_SENSOR_DOWNTIME, MQTT_PORT, PRESSURE_LOG_FILE}, data::{height_from_pressure, process_data, Cache, Calibrations, DataPacket, SharedAlertsVec, SharedCache, SharedCalibrations}
};

#[inline]
pub fn init_client(host: &str) -> (AsyncClient, EventLoop) {
    let username = env::var("MQTT_USERNAME").expect("Error reading MQTT_USERNAME environment variable");
    let password = env::var("MQTT_PASSWORD").expect("Error reading MQTT_PASSWORD environment variable");

    let mut options = MqttOptions::new("crisislab-relay-server", host, MQTT_PORT);
    options.set_keep_alive(Duration::from_secs(10));
    options.set_credentials(username, password);

    AsyncClient::new(options, CHANNEL_CAPACITY)
}

// this polls the event loop and if there's a message it will try to parse it as
// a float and pass it to process_and_cache_data (see data.rs)
pub fn listen(mut event_loop: EventLoop) -> (
    Sender<String>,
    SharedCache,
    SharedAlertsVec,
    SharedCalibrations
) {
    // all of these things will be moved into the task below
    let (broadcast_tx_original, _) = broadcast::channel::<String>(CHANNEL_CAPACITY);
    let cache_original: SharedCache = Arc::new(RwLock::new(Cache::new(CACHE_CAPACITY)));
    let alerts_original: SharedAlertsVec = Arc::new(RwLock::new(Vec::new()));
    let calibrations_original = Arc::new(RwLock::new(Calibrations {
        air_pressure: None,
        resting_water_level: None
    }));

    // the originals will be returned and these will be moved to the task below
    let broadcast_tx = broadcast_tx_original.clone();
    let alerts = alerts_original.clone();
    let cache = cache_original.clone();
    let calibrations = calibrations_original.clone();

    tokio::task::spawn(async move {
        // create a log file for logging data to
        let mut data_log_file: Option<File> = if log_enabled!(log::Level::Trace) {
            match File::create(PRESSURE_LOG_FILE) {
                Ok(file) => Some(file),
                Err(error) => {
                    error!("Could not create log file for pressure values: {}", error);
                    None
                }
            }
        } else {
            None
        };
        
        const CALIBRATION_MSG_PREFIX: &str = "C";
        const AIR: &str = "AIR";
        const WATER: &str = "WATER";

        // how much recent data to use for calibration
        const CALIBRATION_SECONDS: usize = 3;
        const AMOUNT_OF_CALIBRATION_DATA: usize = FREQUENCY * CALIBRATION_SECONDS;

        loop {
            match event_loop.poll().await {
                Ok(event) => {
                    if let Event::Incoming(Packet::Publish(packet)) = event { // if the event was a message
                        let message: String = match String::from_utf8(packet.payload.to_vec()) {
                            Ok(string) => string,
                            Err(error) => {
                                error!("Cound not convert payload from Bytes to String: {}", error);
                                continue;
                            }
                        };

                        debug!("Received message: \"{}\"", message);

                        let mut calibrations_lock = calibrations.write().await;

                        let mut split_message = message.split(" ");

                        // if the first part of the string signifies a calibration message
                        if split_message.next().is_some_and(|str| str == CALIBRATION_MSG_PREFIX) {
                            debug!("Received calibration message");

                            let cache_lock = cache.read().await;
                            let recent_data = cache_lock.last_n(AMOUNT_OF_CALIBRATION_DATA);

                            if let Some(data) = recent_data { // if there was enough data
                                let average_recent_pressure = data
                                    .map(|data| data.get_pressure())
                                    .sum::<f32>() / AMOUNT_OF_CALIBRATION_DATA as f32;

                                // calibrate depending on what the next part of the message is
                                match split_message.next() {
                                    Some(AIR) => {
                                        calibrations_lock.air_pressure = Some(average_recent_pressure);
                                        info!("Calibrated air pressure to {}", average_recent_pressure);
                                    },
                                    Some(WATER) => if let Some(air_pressure_value) = calibrations_lock.air_pressure {
                                        calibrations_lock.resting_water_level = Some(height_from_pressure(
                                            average_recent_pressure,
                                            air_pressure_value
                                        ));
                                        info!("Calibrated resting water level to {}", calibrations_lock.resting_water_level.unwrap());
                                    } else {
                                        warn!("Tried to calibrate resting water level but air pressure hasn't been calibrated yet");
                                    },
                                    _ => warn!("A calibration message was sent, but neither \"{}\" or \"{}\" was specified", AIR, WATER)
                                }
                            } else {
                                warn!("Not enough recent data for calibration, {} seconds of data are required", CALIBRATION_SECONDS);
                            }

                            continue;
                        }

                        let pressure: f32 = match message.parse() {
                            Ok(pressure) => pressure,
                            Err(error) => {
                                warn!("Cound not parse pressure as an f32: {}", error);
                                continue;
                            }
                        };

                        let data = if 
                            calibrations_lock.air_pressure.is_some() &&
                            calibrations_lock.resting_water_level.is_some()
                        {
                            // if calibration has been done then process data
                            process_data(
                                pressure,
                                calibrations_lock.air_pressure.unwrap(),
                                calibrations_lock.resting_water_level.unwrap()
                            ).await
                        } else {
                            // otherwise create an unprocessed
                            DataPacket::unprocessed(pressure)
                        };

                        drop(calibrations_lock);

                        // cache data
                        cache.write().await.write(data);

                        // log to a file if the highest log level is enabled
                        if log_enabled!(log::Level::Trace) {
                            if let Some(ref mut file) = data_log_file {
                                if let Err(error) = file.write(format!("{:?}\n", data).as_bytes()) {
                                    warn!("Error writing data to log file: {}", error);
                                }
                            }
                        }

                        // stringify data and send to websocket connection handlers which will
                        // forward it to clients
                        if let Err(error) = broadcast_tx.send(serde_json::to_string(&data).unwrap()) {
                            warn!("Could not broadcast processed data to WebSocket connection handlers: {}", error);
                        }

                        if let Some(alert) = check_for_alert(&cache, &alerts).await {
                            if let Err(error) = broadcast_tx.send(serde_json::to_string(&alert).unwrap()) {
                                warn!("Could not broadcast alert to WebSocket connection handlers: {}", error);
                            }   
                        }
                    }
                }
                Err(error) => {
                    error!("Error polling MQTT event loop: {:?}", error);
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            }
        }
    });

    let broadcast_tx = broadcast_tx_original.clone();
    let cache = cache_original.clone();

    tokio::task::spawn(async move {
        loop {
            if let Some(previous_data) = cache.read().await.last() { // get last data packet
                // if the last message was long enough ago
                if previous_data.get_timestamp().elapsed() > MAX_SENSOR_DOWNTIME {
                    // notify that the sensor has been offline for longer than expected
                    let message = serde_json::to_string(&json!({ "sensor_offline": true })).unwrap();
                    if let Err(error) = broadcast_tx.send(message) {
                        warn!("Could not notify WebSocket connections of sensor being offline: {}", error);
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    });

    (broadcast_tx_original, cache_original, alerts_original, calibrations_original)
}
