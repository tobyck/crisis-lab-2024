/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* */

use std::{env, time::Duration};

use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet};

use crate::{data::{self, SharedCache}, helpers, HERTZ};

pub fn init_client(host: &str) -> (AsyncClient, EventLoop) {
    // get port, username and password from environment variables
    let mqtt_port = helpers::get_env_port("MQTT_PORT");
    let username = env::var("USERNAME").expect("Must set USERNAME environment variable");
    let password = env::var("PASSWORD").expect("Must set PASSWORD environment variable");

    let mut options = MqttOptions::new("crisislab-relay-server", host, mqtt_port);
    options.set_keep_alive(Duration::from_secs(10));
    options.set_credentials(username, password);

    // create our client and retain 10s of messages in the bounded channel
    AsyncClient::new(options, HERTZ * 10)
}

// this polls the event loop and if there's a message it will try to parse it as
// a float and pass it to process_and_cache_data (see data.rs)
pub fn listen_for_data(mut event_loop: EventLoop, cache: SharedCache) {
    tokio::task::spawn(async move {
        loop {
            match event_loop.poll().await {
                Ok(event) => {
                    if let Event::Incoming(Packet::Publish(packet)) = event { // if the event was a message
                        match String::from_utf8(packet.payload.to_vec()) { // convert payload from Bytes
                            Ok(payload) => match payload.parse::<f32>() {
                                Ok(pressure) => data::process_and_cache_data(pressure, &cache),
                                Err(error) => println!("Error parsing pressure as a u32: {}", error)
                            },
                            Err(error) => println!("Could not convert payload from Bytes to String: {}", error)
                        }
                    }
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}
