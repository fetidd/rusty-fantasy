use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, http::HeaderValue, response::IntoResponse, routing::get, Router};
use futures_util::{SinkExt, StreamExt};
use shared::messaging::{SystemRequest, SystemResponse};
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (tx, _) = channel(100);
    let app = app(tx);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app(tx: Sender<SystemResponse>) -> Router {
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
    State(tx): State<Sender<SystemResponse>>,
) -> impl IntoResponse {
    tracing::info!("Upgrading to WebSocket");
    ws.on_upgrade(|socket| handle_socket(socket, tx))
}

async fn handle_socket(socket: WebSocket, tx: Sender<SystemResponse>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx: Receiver<SystemResponse> = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            tracing::info!("Sending message: {:?}", msg);
            sender.send(Message::from(serde_json::to_string(&msg).unwrap())).await.unwrap();
        }
    });

    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(content) => {
                    let msg: SystemRequest = serde_json::from_str(&content).unwrap();
                    tracing::info!("Received message: {:?}", msg);
                    match msg {
                        SystemRequest::Chat { username, content, role } => {
                            let response = SystemResponse::Chat {
                                username,
                                content,
                                role
                            };
                            tx.send(response).unwrap();
                        },
                        SystemRequest::Roll { username, tags } => {
                            let roll = (rand::random::<u8>() % 6 + 1, rand::random::<u8>() % 6 + 1);
                            let roll_total = roll.0 + roll.1;
                            let total = apply_tags_to_roll(roll_total, &tags);
                            let response = SystemResponse::Roll {
                                dice_values: roll,
                                username,
                                tags,
                                total
                            };
                            tx.send(response).unwrap();
                        },
                        _ => (),
                    }
                },
                Message::Close(_) => break,
                _ => ()
            }
        }
    }
}

fn apply_tags_to_roll(roll_total: u8, tags: &shared::messaging::TagMap) -> u8 {
    roll_total + 3 // placeholder implementation
}