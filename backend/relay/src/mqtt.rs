/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This file handles everything MQTT related -- intialising the client with the
* appropriate options, polling the event loop for messages, passing the message
* to the function in data.rs to process the data, and sending it to the
* WebSocket handlers.
* */

use std::{env, sync::Arc, time::Duration};

use log::{error, warn};
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet};
use tokio::sync::{broadcast::{self, Sender}, RwLock};

use crate::{
    config::{CACHE_CAPACITY, CHANNEL_CAPACITY, MQTT_PORT},
    data::{process_data, Cache, DataPacket, SharedAlertsVec, SharedCache}
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
pub fn listen(mut event_loop: EventLoop) -> (Sender<DataPacket>, SharedCache, SharedAlertsVec) {
    // all of these things will be moved into the task below
    let (broadcast_tx, _) = broadcast::channel::<DataPacket>(CHANNEL_CAPACITY);
    let alerts: SharedAlertsVec = Arc::new(RwLock::new(Vec::new()));
    let cache: SharedCache = Arc::new(RwLock::new(Cache::new(CACHE_CAPACITY)));

    // so we need to make clones to be returned by this function
    let broadcast_tx_clone = broadcast_tx.clone();
    let alerts_clone = alerts.clone();
    let cache_clone = cache.clone();

    tokio::task::spawn(async move {
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

                        let pressure: f32 = match message.parse() {
                            Ok(pressure) => pressure,
                            Err(error) => {
                                error!("Cound not parse pressure as an f32: {}", error);
                                continue;
                            }
                        };

                        let data: DataPacket = process_data(pressure, &cache, &alerts).await;

                        // send data to websocket handlers
                        if let Err(error) = broadcast_tx.send(data) {
                            warn!("Could not broadcast processed data to WebSocket connection handlers: {}", error);
                        }
                    }
                }
                Err(error) => {
                    error!("Error: {:?}", error);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });

    (broadcast_tx_clone, cache_clone, alerts_clone)
}
