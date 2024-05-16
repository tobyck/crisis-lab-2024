use futures_util::{FutureExt, StreamExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    let echo = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let routes = echo;

    warp::serve(routes)
        .tls()
        .cert_path("tls/cert.pem")
        .key_path("tls/key.rsa")
        .run(([127, 0, 0, 1], 8080)).await;
}
