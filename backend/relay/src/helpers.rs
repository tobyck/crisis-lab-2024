/*
* Author: Toby Connor-Kebbell
* Date: May 2024
*
* This is where all the things that I don't feel fit anywhere else go.
* */

use std::{convert::Infallible, env};

use warp::{http::StatusCode, reject::Rejection, reply::{self, Reply}};
use serde::Serialize;

// JSON serialisable struct which will be the reply for all rejections
#[derive(Serialize)]
struct RejectionMessage {
    error: String
}

pub async fn handle_rejection(error: Rejection) -> Result<impl Reply, Infallible> {
    let status: StatusCode;
    let message: &str;

    if error.is_not_found() {
        status = StatusCode::NOT_FOUND;
        message = "Not found";
    } else {
        status = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error"
    }

    Ok(reply::with_status(
        reply::json(&RejectionMessage { error: message.to_string() }),
        status
    ))
} 

pub fn get_port_from_env(name: &str) -> u16 {
    env::var(name)
        .expect("Error reading environment variable")
        .parse::<u16>()
        .expect("Unable to parse port as a u16")
}
