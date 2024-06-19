/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This is the entry point for the relay server. All it really does is call
* functions from other modules to start everything up.
* */

use std::{convert::Infallible, env};

use config::MQTT_TOPIC;
use rumqttc::QoS;
use warp::{reject::Rejection, reply::{self, Reply}, Filter, http::StatusCode};

mod config;
mod mqtt;
mod ws;
mod data;
mod alert;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    // initialise the client and subscribe to the topic that the sensor will be publishing to
    let (client, event_loop) = mqtt::init_client("localhost");
    client.subscribe(MQTT_TOPIC, QoS::ExactlyOnce).await.unwrap();

    // start listening for messages in a separate task and return some other initialised
    // objects that the websocket connection handlers will need
    let (broadcast_tx, cache, alerts, calibrations) = mqtt::listen(event_loop);

    let server_addr = [0, 0, 0, 0];
    let ws_port = get_env_port("WS_PORT");
    let wss_port = get_env_port("WSS_PORT");

    let cert_path = env::var("CERT_PATH").expect("Must set CERT_PATH");
    let key_path = env::var("KEY_PATH").expect("Must set KEY_PATH");

    // .recover takes a handler for creating replies when something goes wrong
    let route = ws::route(broadcast_tx, cache, alerts, calibrations).recover(handle_rejection);

    // instance of the server with tls
    let secure_sever = warp::serve(route.clone())
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run((server_addr, wss_port));

    // instance without tls
    let insecure_server = warp::serve(route)
        .run((server_addr, ws_port));

    // run both at the same time
    futures::future::join(secure_sever, insecure_server).await;
}

async fn handle_rejection(error: Rejection) -> Result<impl Reply, Infallible> {
    if error.is_not_found() {
        Ok(reply::with_status(reply::reply(), StatusCode::NOT_FOUND))
    } else {
        Ok(reply::with_status(reply::reply(), StatusCode::INTERNAL_SERVER_ERROR))
    }
}

fn get_env_port(name: &str) -> u16 {
    env::var(name)
        .expect(format!("Must specify WebSocket port in {} environment variable", name).as_str())
        .parse::<u16>()
        .expect(format!("Could not parse {} as a u16", name).as_str())
}
