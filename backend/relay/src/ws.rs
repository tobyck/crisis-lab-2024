/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* 
* This file is responsible for relaying data from the cache to dashboard clients
* over WebSockets.
* */

use std::time::Duration;

use futures::{SinkExt, StreamExt, TryFutureExt};
use warp::{filters::ws::{Message, WebSocket}, reject::Rejection, reply::Reply, Filter};
use log::warn;

use crate::{data::SharedCache, FREQUENCY};

pub async fn handle_connection(websocket: WebSocket, cache: SharedCache) {
    let (mut tx, _) = websocket.split();

    tokio::task::spawn(async move {
        loop {
            let cache_lock = cache.read().await;

            if let Some(data) = cache_lock.read_last() {
                tx.send(Message::text(serde_json::to_string(&data).unwrap()))
                    .unwrap_or_else(|error| {
                        warn!("Error trying to send data: {}", error);
                    }).await;
            }

            drop(cache_lock);

            tokio::time::sleep(Duration::from_millis(1000 / FREQUENCY)).await;
        }
    });
}

// this returns the Warp route that will be served
pub fn route(cache: SharedCache) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // a filter which allows us to pass the cache to the connection handlemove r
    let with_cache = warp::any().map(move || cache.clone());

    warp::path::end() // this means the path '/'
        .and(warp::ws())
        .and(with_cache)
        .map(|route: warp::ws::Ws, cache: SharedCache| {
            route.on_upgrade(move |websocket| handle_connection(websocket, cache))
        })
}
