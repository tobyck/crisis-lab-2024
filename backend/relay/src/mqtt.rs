/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This file handles everything MQTT related -- initialising the client with the
* appropriate options, polling the event loop for messages, passing the message
* to the function in data.rs to process the data, and sending it to the
* WebSocket handlers.
* */

use std::{env, sync::Arc, time::{Duration, Instant}};

use log::{debug, info, warn, error};
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet};
use serde_json::json;
use tokio::sync::{broadcast::{self, Sender}, RwLock};

use crate::{
    alert::check_for_alert, config::{
        CACHE_CAPACITY, CHANNEL_CAPACITY, FREQUENCY,
        MAX_SENSOR_DOWNTIME, MQTT_PORT
    },
    data::{
        height_from_pressure, process_data, Cache, Calibrations,
        DataPacket, SharedAlertsVec, SharedCache, SharedCalibrations
    }
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

async fn handle_mqtt_message(
    message: String,
    broadcast_tx: &Sender<String>,
    cache: &SharedCache,
    alerts: &SharedAlertsVec,
    calibrations: &SharedCalibrations,
    alert_threshold: f32,
    air_calibration_timestamp: &mut Option<Instant>
) {
    // the messages for calibration are "C AIR" and "C WATER"
    const CALIBRATION_MSG_PREFIX: &str = "C";
    const AIR: &str = "AIR";
    const WATER: &str = "WATER";

    // these messages are in the form `T <time_millis>` and can be used to time latency
    const TIMESTAMP_MSG_PREFIX: &str = "T";

    // how much recent data to use for calibration
    const CALIBRATION_SECONDS: usize = 3;
    const AMOUNT_OF_CALIBRATION_DATA: usize = FREQUENCY * CALIBRATION_SECONDS;

    let mut calibrations_lock = calibrations.write().await;
    let mut split_message = message.split(" ");
    let first_word = split_message.next();

    if first_word.is_none() {
        warn!("Got empty message. Doing nothing");
    }

    match first_word.unwrap() {
        CALIBRATION_MSG_PREFIX => {
            debug!("Received calibration message");

            let cache_lock = cache.read().await;
            let recent_data = cache_lock.last_n(AMOUNT_OF_CALIBRATION_DATA);

            if recent_data.is_none() {
                warn!("Not enough recent data for calibration, {} seconds of data are required", CALIBRATION_SECONDS);
            }

            let first_of_recent_data = cache_lock.at(cache_lock.len() - AMOUNT_OF_CALIBRATION_DATA);

            let average_recent_pressure = recent_data
                .unwrap()
                .map(|data| data.get_pressure())
                .sum::<f32>() / AMOUNT_OF_CALIBRATION_DATA as f32;

            // calibrate depending on what the next part of the message is
            match split_message.next() {
                Some(AIR) => {
                    *air_calibration_timestamp = Some(Instant::now());
                    calibrations_lock.air_pressure = Some(average_recent_pressure);
                    info!("Calibrated air pressure to {}", average_recent_pressure);
                },
                Some(WATER) => if let Some(air_pressure_value) = calibrations_lock.air_pressure {
                    // this checks that the data being used for water calibration doesn't
                    // overlap with the data that was used for the air pressure calibration
                    if first_of_recent_data.is_some_and(
                        // air calibration must be done first, so air_calibration_timestamp
                        // must be Some at this point and .unwrap() is ok
                        |data_packet| data_packet.get_timestamp() > air_calibration_timestamp.unwrap()
                    ) {
                        calibrations_lock.resting_water_level = Some(height_from_pressure(
                            average_recent_pressure,
                            air_pressure_value
                        ));

                        info!("Calibrated resting water level to {}", calibrations_lock.resting_water_level.unwrap());
                    } else {
                        warn!("Not enough data for resting water level calibration");
                    }
                } else {
                    warn!("Tried to calibrate resting water level but air pressure hasn't been calibrated yet");
                },
                _ => warn!("A calibration message was sent, but neither \"{}\" or \"{}\" was specified", AIR, WATER)
            }
        },
        TIMESTAMP_MSG_PREFIX => {
            info!("Got timestamp message");

            if let Some(timestamp) = split_message.next() {
                match timestamp.parse::<u64>() {
                    Ok(timestamp) => {
                        // we decided not to put any dummy data processing here because the time it
                        // takes is negligible

                        let message = json!({
                            "test_timestamp": timestamp
                        }).to_string();

                        // send the timestamp to the frontend which can work out how much time has
                        // passed since the initial message was sent
                        if let Err(error) = broadcast_tx.send(message) {
                            warn!("Could not broadcast timestamp to WebSocket connection handlers: {}", error);
                        }
                    },
                    Err(error) => warn!("Error parsing timestamp: {}", error)
                }
            }
        },
        // if the message wasn't anything special then treat it as a pressure value
        _ => {
            let pressure: f32 = match message.parse() {
                Ok(pressure) => pressure,
                Err(error) => {
                    warn!("Cound not parse pressure as an f32: {}", error);
                    return;
                }
            };

            let calibrations_complete = calibrations_lock.air_pressure.is_some() && calibrations_lock.resting_water_level.is_some();

            let data = if calibrations_complete {
                process_data(
                    pressure,
                    calibrations_lock.air_pressure.unwrap()
                ).await
            } else {
                DataPacket::unprocessed(pressure)
            };

            // because we're about to free calibrations_lock (see comment below), we need to
            // extract this here before we pass it to check_for_alert
            let resting_water_level = calibrations_lock.resting_water_level;

            // if we don't drop this write lock now, then the code in the websocket connection
            // handlers (which will be invoked very soon by messages on the broadcast channel)
            // won't be able to obtain read locks on the calibrations because of this write lock
            // still being held.
            drop(calibrations_lock);

            cache.write().await.write(data);

            // stringify data and send to websocket connection handlers which will forward it to clients
            if let Err(error) = broadcast_tx.send(serde_json::to_string(&data).unwrap()) {
                warn!("Could not broadcast processed data to WebSocket connection handlers: {}", error);
            }

            if let Some(resting_water_level) = resting_water_level {
                let alert = check_for_alert(
                    alert_threshold,
                    resting_water_level,
                    &cache,
                    &alerts
                ).await;

                if let Some(alert) = alert {
                    if let Err(error) = broadcast_tx.send(serde_json::to_string(&alert).unwrap()) {
                        warn!("Could not broadcast alert to WebSocket connection handlers: {}", error);
                    }
                }
            }
        }
    }
}

// this polls the event loop and if there's a message it will try to parse it as
// a float and pass it to process_and_cache_data (see data.rs)
pub fn start_listening(mut event_loop: EventLoop, alert_threshold: f32) -> (
    Sender<String>,
    SharedCache,
    SharedAlertsVec,
    SharedCalibrations
) {
    let (broadcast_tx_original, _) = broadcast::channel::<String>(CHANNEL_CAPACITY);
    let cache_original: SharedCache = Arc::new(RwLock::new(Cache::new(CACHE_CAPACITY)));
    let alerts_original: SharedAlertsVec = Arc::new(RwLock::new(Vec::new()));
    let calibrations_original = Arc::new(RwLock::new(Calibrations {
        air_pressure: None,
        resting_water_level: None
    }));

    // the originals will be returned and these will be moved to the task below
    let broadcast_tx = broadcast_tx_original.clone();
    let cache = cache_original.clone();
    let alerts = alerts_original.clone();
    let calibrations = calibrations_original.clone();

    tokio::task::spawn(async move {
        // this holds the time that the first calibration was done. this is used to ensure that
        // when the second calibration is done the data is different to that used for the first
        let mut air_calibration_timestamp: Option<Instant> = None;

        loop {
            match event_loop.poll().await {
                Ok(event) => {
                    // mqtt sends many different types of packets, so this guard makes sure that
                    // we're getting an actual message from the sensor
                    if let Event::Incoming(Packet::Publish(packet)) = event {
                        let message: String = match String::from_utf8(packet.payload.to_vec()) {
                            Ok(string) => string,
                            Err(error) => {
                                error!("Cound not convert payload from Bytes to String: {}", error);
                                continue;
                            }
                        };

                        debug!("Received message: \"{}\"", message);

                        handle_mqtt_message(
                            message,
                            &broadcast_tx,
                            &cache,
                            &alerts,
                            &calibrations,
                            alert_threshold,
                            &mut air_calibration_timestamp
                        ).await;
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

    // spawn a task which periodically checks if the sensor is online
    tokio::task::spawn(async move {
        loop {
            if let Some(previous_data_packet) = cache.read().await.last() {
                // if the last message was long enough ago
                if previous_data_packet.get_timestamp().elapsed() > MAX_SENSOR_DOWNTIME {
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

    // return data to be passed to websocket connection handlers
    (broadcast_tx_original, cache_original, alerts_original, calibrations_original)
}

#[cfg(test)]
mod benchmarks {
    use test::Bencher;
    use std::{hint::black_box, sync::Arc};
    use tokio::sync::{broadcast, RwLock};
    use futures::executor::block_on;

    use crate::{config::{CACHE_CAPACITY, CHANNEL_CAPACITY}, data::{Cache, Calibrations, SharedAlertsVec, SharedCache}};

    use super::handle_mqtt_message;

    #[bench]
    fn bench_handle_mqtt_message(b: &mut Bencher) {
        black_box({
            let (broadcast_tx, _broadcast_rx) = broadcast::channel::<String>(CHANNEL_CAPACITY);
            let cache: SharedCache = Arc::new(RwLock::new(Cache::new(CACHE_CAPACITY)));
            let alerts: SharedAlertsVec = Arc::new(RwLock::new(Vec::new()));

            let calibrations = Arc::new(RwLock::new(Calibrations {
                air_pressure: None,
                resting_water_level: None
            }));

            b.iter(|| {
                block_on(handle_mqtt_message(
                    1000.to_string(),
                    &broadcast_tx,
                    &cache,
                    &alerts,
                    &calibrations,
                    5.0,
                    &mut None
                ));
            })
        })
    }
}
