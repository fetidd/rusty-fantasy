use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, http::HeaderValue, response::IntoResponse, routing::get, Router};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let (tx, _) = channel(100);
    let app = app(tx);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app(tx: Sender<String>) -> Router {
    let origins = vec![
        "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
    ];
    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods(Any);

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/message", get(message_handler))
        .with_state(tx)
        .layer(cors_layer)
}

async fn message_handler(
    ws: WebSocketUpgrade,
    State(tx): State<Sender<String>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, tx))
}

async fn handle_socket(socket: WebSocket, tx: Sender<String>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx: Receiver<String> = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            sender.send(Message::from(msg)).await.unwrap();
        }
    });

    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(content) => {
                    tx.send(content.to_string()).unwrap();
                },
                Message::Close(_) => break,
                _ => ()
            }
        }
    }
}