/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This file is responsible for receiving data packets on the broadcast channel
* and relying it to clients over WebSockets. It also sends previous data from
* the cache and all the previous alerts that have occurred since the server has
* been up when a new client connects.
* */

use log::{debug, error, info, warn};
use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::broadcast::{Receiver, Sender};
use warp::{filters::ws::{Message, WebSocket}, reject::Rejection, reply::Reply, Filter};

use crate::data::{InitialDataPacket, SharedAlertsVec, SharedCache, SharedCalibrations};

pub async fn handle_connection(
    websocket: WebSocket,
    mut broadcast_rx: Receiver<String>, // this is where the handler will receive messages
    cache: SharedCache,
    alerts: SharedAlertsVec,
    calibrations: SharedCalibrations
) {
    info!("Client connected to WebSocket");

    tokio::task::spawn(async move {
        let (mut websocket_tx, mut websocket_rx) = websocket.split();

        // send initial previous data and alerts upon connection
        websocket_tx.send(Message::text(serde_json::to_string(&InitialDataPacket {
            previous_data: cache.read().await.to_vec(),
            previous_alerts: alerts.read().await.to_vec(),
            calibrations: calibrations.read().await.clone()
        }).unwrap()))
            .unwrap_or_else(|error| {
                warn!("Failed to send initial data packet over websocket: {}", error);
            }).await;

        loop {
            tokio::select! {
                // if the client sends an empty message then it's disconnected; end the loop
                msg = websocket_rx.next() => {
                    if msg.is_none() {
                        info!("Client disconnected from WebSocket");
                        break;
                    }
                },
                // if we receive a message from the broadcast channel, send it to the client
                data = broadcast_rx.recv() => {
                    debug!("Got message on broadcast channel");
                    match data {
                        Ok(data) => {
                            websocket_tx.send(Message::text(data.clone()))
                                .unwrap_or_else(|error| {
                                    error!("Failed to send data packet over websocket: {}", error);
                                }).await;
                        },
                        Err(error) => error!("Error reading message from broadcast channel: {}", error)
                    }
                }
            }
        }
    });
}

// this returns the Warp route that will be served
pub fn route(
    broadcast_tx: Sender<String>,
    cache: SharedCache,
    alerts: SharedAlertsVec,
    calibrations: SharedCalibrations
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // a warp filter that allows us to pass other things to the connection handler
    let with_args = warp::any().map(move || (
        broadcast_tx.subscribe(),
        cache.clone(),
        alerts.clone(),
        calibrations.clone()
    ));

    warp::path::end() // this means the path '/'
        .and(warp::ws())
        .and(with_args)
        .map(|route: warp::ws::Ws, (broadcast_rx, cache, alerts, calibrations)| {
            route.on_upgrade(move |websocket| handle_connection(
                websocket,
                broadcast_rx,
                cache,
                alerts,
                calibrations
            ))
        })
}
