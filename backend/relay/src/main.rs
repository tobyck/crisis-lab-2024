/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This is the entry point for the relay server. All it really does is call
* functions from other modules to start everything up.
* */

use rumqttc::QoS;
use warp::Filter;

mod data;
mod helpers;
mod mqtt;
mod ws;

pub const FREQUENCY: u64 = 25;
pub const CHANNEL_CAPACITY: usize = FREQUENCY as usize * 10;
pub const CACHE_CAPACITY: usize = FREQUENCY as usize * 20;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    // initialise the client and subscribe to the topic that the senor will be publishing to
    let (client, event_loop) = mqtt::init_client("localhost");
    client.subscribe("data", QoS::AtMostOnce).await.unwrap();

    // start listening for messages in a seperate task and return some other initalised
    // objects that the websocket connection handlers will need
    let (broadcast_tx, cache, alerts) = mqtt::listen(event_loop);

    // serve the websocket route and pass in said objects, and use a handler to
    // reply when something goes wrong
    warp::serve(ws::route(broadcast_tx, cache, alerts).recover(helpers::handle_rejection))
        .tls()
        .cert_path("tls/cert.crt")
        .key_path("tls/cert.key")
        .run(([0, 0, 0, 0], helpers::get_port_from_env("WS_PORT")))
        .await;
}
