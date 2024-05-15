use dotenv::dotenv;
use std::env;

use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let site_dir = env::var("SITE_DIR").expect("Error reading SITE_DIR environment variable");

    let assets = warp::fs::dir(site_dir.clone());

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(site_dir + "index.html"));

    let routes = index.or(assets);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
