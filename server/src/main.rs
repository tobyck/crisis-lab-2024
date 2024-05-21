use std::env;

use warp::Filter;

mod helpers;
mod ws;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let port = env::var("PORT")
        .expect("Error reading PORT environment variable")
        .parse::<u16>()
        .expect("Unable to parse PORT as a u16");

    warp::serve(ws::route().await.recover(helpers::handle_rejection))
        .tls()
        .cert_path("tls/cert.pem")
        .key_path("tls/key.pem")
        .run(([0, 0, 0, 0], port)).await;
}
