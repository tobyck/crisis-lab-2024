use std::env;

use log::{info, warn};
use warp::{filters::ws::WebSocket, reject::{self, Reject, Rejection}, reply::Reply, Filter};

#[derive(Debug)]
pub enum AuthError {
    IncorrectKey,
    InvalidFormat,
    NoKeyProvided,
    ServerSpecifiedNoKey
}

impl Reject for AuthError {}

// Creates a filter which insures that the client has the correct auth key
// (Idk why Clone is in the type signature but for some reason it needs to be there)
async fn ensure_authorisation() -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    warp::header::optional("Authorization").and_then(|auth_header: Option<String>| async move {
        if let Some(header) = auth_header {
            info!("Got auth header, verifying: {}", header);

            // the header should come as "<auth-scheme> <auth_params>"
            let parts: Vec<&str> = header.split(" ").collect();

            if parts.len() != 2 || parts[0] != "Basic" {
                return Err(reject::custom(AuthError::InvalidFormat));
            }

            if let Ok(key) = env::var("AUTH_KEY") { // if the auth key env var exists
                return // return Ok if the key is correct, otherwise reject
                    if parts[1] == key { Ok(()) }
                    else { Err(reject::custom(AuthError::IncorrectKey)) }
            } else {
                warn!("AUTH_KEY environment variable not found");
            }

            return Err(reject::custom(AuthError::ServerSpecifiedNoKey));
        }

        Err(reject::custom(AuthError::NoKeyProvided))
    })
}

async fn handle_connection(_websocket: WebSocket) {
    
}

pub async fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("send-data")
        .and(warp::ws())
        .and(ensure_authorisation().await)
        // the ignored arg is the result of the ensure_authorisation filter
        .map(|route: warp::ws::Ws, _| {
           route.on_upgrade(handle_connection)
        })
}
