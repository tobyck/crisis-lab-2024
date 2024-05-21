use std::convert::Infallible;

use serde::Serialize;
use warp::{http::StatusCode, reject::{Reject, Rejection}, reply};

#[derive(Debug)]
pub enum AuthError {
    FailedToReadAuthMessage,
    InvalidIntentions,
    MissingFields,
    ServerError
}

impl Reject for AuthError {}

// JSON serialisable struct which will be the reply for all rejections
#[derive(Serialize)]
struct RejectionMessage {
    message: String
}

// Create a reply with a status given a rejection object
pub async fn handle_rejection(error: Rejection) -> Result<impl reply::Reply, Infallible> {
    let status: StatusCode;
    let message: &str;

    if error.is_not_found() {
        status = StatusCode::NOT_FOUND;
        message = "Not found";
    } else {
        status = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Alright that's my bad. (Internal server error)"
    }

    Ok(reply::with_status(
        reply::json(&RejectionMessage { message: message.to_string() }), 
        status
    ))
}
