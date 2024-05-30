/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* */

use log::error;
use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::broadcast::{Receiver, Sender};
use warp::{filters::ws::{Message, WebSocket}, reject::Rejection, reply::Reply, Filter};

use crate::data::{DataPacket, SharedAlertsVec, SharedCache};

#[allow(unused_variables)]
pub async fn handle_connection(
    websocket: WebSocket,
    mut broadcast_rx: Receiver<DataPacket>,
    cache: SharedCache,
    alerts: SharedAlertsVec
) {
    let (mut websocket_tx, _) = websocket.split();

    tokio::task::spawn(async move {
        loop {
            match broadcast_rx.recv().await {
                Ok(data) => {
                    websocket_tx.send(Message::text(serde_json::to_string(&data).unwrap()))
                        .unwrap_or_else(|error| {
                            error!("Error trying to send data: {}", error);
                        }).await;
                },
                Err(error) => error!("Error reading message from broadcast channel: {}", error)
            } 
        }
    });
}

// this returns the Warp route that will be served
pub fn route(
    broadcast_tx: Sender<DataPacket>,
    cache: SharedCache,
    alerts: SharedAlertsVec
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let with_args = warp::any().map(move || (broadcast_tx.subscribe(), cache.clone(), alerts.clone()));

    warp::path::end() // this means the path '/'
        .and(warp::ws())
        .and(with_args)
        .map(|route: warp::ws::Ws, (broadcast_rx, cache, alerts)| {
            route.on_upgrade(move |websocket| handle_connection(websocket, broadcast_rx, cache, alerts))
        })
}
