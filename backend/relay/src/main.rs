/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This is the entry point for the relay server. All it really does is call
* functions from other modules to start everything up.
* */

use std::convert::Infallible;

use rumqttc::QoS;
use warp::{reject::Rejection, reply::{self, Reply}, Filter, http::StatusCode};

mod config;
mod mqtt;
mod ws;
mod data;

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
    warp::serve(ws::route(broadcast_tx, cache, alerts).recover(handle_rejection))
        .run(([0, 0, 0, 0], config::WS_PORT))
        .await;
}

async fn handle_rejection(error: Rejection) -> Result<impl Reply, Infallible> {
    if error.is_not_found() {
        Ok(reply::with_status(reply::reply(), StatusCode::NOT_FOUND))
    } else {
        Ok(reply::with_status(reply::reply(), StatusCode::INTERNAL_SERVER_ERROR))
    }
}
