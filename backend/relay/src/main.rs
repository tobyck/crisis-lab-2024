/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* 
* This is the entry point for the relay server. All it really does is call
* functions from other modules to start everything up.
* */

use std::sync::Arc;
use tokio::sync::RwLock;

use rumqttc::QoS;
use warp::Filter;

mod mqtt;
mod ws;
mod data;
mod helpers;

// this is always being multiplied by something then being passed into a
// function that's expecting a usize, so it's a usize here so that we don't have
// to cast it.
pub const FREQUENCY: u64 = 25;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let (client, event_loop) = mqtt::init_client("localhost");
    client.subscribe("data", QoS::AtMostOnce).await.unwrap();

    let cache: data::SharedCache = Arc::new(RwLock::new(data::Cache::new(FREQUENCY as usize * 20)));
    mqtt::listen_for_data(event_loop, cache.clone());

    // `recover` will pass rejections to a handler to turn them into normal replies
    warp::serve(ws::route(cache).recover(helpers::handle_rejection))
        .tls()
        .cert_path("tls/cert.crt")
        .key_path("tls/cert.key")
        .run((
            [0, 0, 0, 0],
            helpers::get_port_from_env("WS_PORT")
        )).await;
}
