/*
* Author: Toby Connor-Kebbell
* Date: May 2024
* 
* This file is responsible for relaying data from the cache to dashboard
* clients over WebSockets.
* */

use warp::{filters::ws::WebSocket, reject::Rejection, reply::Reply, Filter};

pub async fn handle_connection(_websocket: WebSocket) {
    // send data from cache here
}

// this is the Warp route that will be served
pub fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path::end() // this means the path '/'
        .and(warp::ws())
        .map(|route: warp::ws::Ws| {
            route.on_upgrade(move |websocket| handle_connection(websocket))
        })
}
