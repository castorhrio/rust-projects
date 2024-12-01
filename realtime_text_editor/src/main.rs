use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use warp::filters::ws::Message;
use warp::ws::WebSocket;
use warp::Filter;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EditMessage {
    content: String,
    cursor_position: usize,
}

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(100);

    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || tx.clone()))
        .map(|ws: warp::ws::Ws, tx| ws.on_upgrade(move |socket| handle_connection(socket, tx)));

    println!("server running at http://localhost:8080");

    let html_route = warp::path::end().and(warp::fs::file("./index.html"));
    let routes = websocket_route.or(html_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_connection(ws: WebSocket, tx: broadcast::Sender<EditMessage>) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    let mut rx = tx.subscribe();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(text) = serde_json::to_string(&msg) {
                if ws_tx.send(Message::text(text)).await.is_err() {
                    break;
                }
            }
        }
    });

    let rev_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            if let Ok(text) = msg.to_str() {
                if let Ok(edit) = serde_json::from_str::<EditMessage>(text) {
                    tx.send(edit).unwrap();
                }
            }
        }
    });

    tokio::select! {
        _ = send_task =>(),
        _ = rev_task=>(),
    }
}
