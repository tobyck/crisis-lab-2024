use std::sync::{Arc, RwLock};

use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use warp::{filters::ws::{Message, WebSocket}, reject::{self, Rejection}, reply::{Reply, Response}, Filter};

use crate::helpers::AuthError;

// This is the object you read messages from on a WebSocket but wrapped in Arc and Mutex so that
// it's safe to share and mutate between Tokio tasks, and Option so that it can be None initially
type DataSource = Arc<RwLock<Option<SplitStream<WebSocket>>>>;

#[allow(unused)]
async fn handle_connection(
    websocket: WebSocket,
    data_source: DataSource
) {
    let (mut send, mut messages) = websocket.split();

    if let Some(result) = messages.next().await {
        let message = result.unwrap_or_else(async |error| websocket.close().await.unwrap()).to_str();
    }
}

pub async fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let original_data_source: DataSource = Arc::new(RwLock::new(None));

    // filter that passes a clone of the data source to each WebSocket handler
    let with_data_source = warp::any().map(move || original_data_source.clone());

    warp::path::end()
        .and(warp::ws())
        .and(with_data_source)
        .map(|route: warp::ws::Ws, data_source_clone: DataSource| {
            route.on_upgrade(move |websocket| handle_connection(websocket, data_source_clone))
        })
}
