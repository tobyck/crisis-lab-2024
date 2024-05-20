mod send_data_ws;

use send_data_ws::AuthError;
use std::{convert::Infallible, env};
use warp::{http::StatusCode, reject::Rejection, reply, Filter};
use serde::Serialize;

// JSON serialisable struct which will be the reply for all rejections
#[derive(Serialize)]
struct RejectionMessage {
    message: String
}

// Create a reply with a status given a rejection object
async fn handle_rejection(error: Rejection) -> Result<impl reply::Reply, Infallible> {
    let mut status: StatusCode;
    let message: &str;

    if error.is_not_found() {
        status = StatusCode::NOT_FOUND;
        message = "Not found";
    } else if let Some(auth_error) = error.find::<AuthError>() { // "find" if the reason was an AuthError
        status = StatusCode::UNAUTHORIZED;
        message = match auth_error {
            AuthError::IncorrectKey => "Incorrect auth key.",
            AuthError::InvalidFormat => "Auth credentials are in the wrong format.",
            AuthError::NoKeyProvided => "No auth key provided.",
            AuthError::ServerSpecifiedNoKey => {
                status = StatusCode::INTERNAL_SERVER_ERROR;
                "Looks like not even the server knows what the key should be. Might wanna tell someone/do something about that." 
            }

        }
    } else {
        status = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Alright that's my bad. (Internal server error)"
    }

    Ok(reply::with_status(
        reply::json(&RejectionMessage { message: message.to_string() }), 
        status
    ))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let routes = send_data_ws::route().await;

    let port = env::var("PORT")
        .expect("Error reading PORT environment variable")
        .parse::<u16>()
        .expect("Unable to parse PORT as a u16");

    warp::serve(routes.recover(handle_rejection))
        .tls()
        .cert_path("tls/cert.pem")
        .key_path("tls/key.pem")
        .run(([0, 0, 0, 0], port)).await;
}
