mod components;

use dioxus::prelude::*;

use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};

use shared::messaging::{SystemResponse, SystemRequest};

fn main() {
    launch(|| {
        tracing::info!("Starting Client");
        let (ws_coroutine, message_list) = set_up_socket();
        let client_state = ClientState {
            messages: message_list,
            username: use_signal(|| String::new()),
        };
        let _ = use_context_provider(|| ws_coroutine);
        let _ = use_context_provider(|| client_state); // can this just be global state?
        rsx! { components::client::Client {} }
    });
}

#[derive(Clone)]
struct ClientState {
    messages: Signal<Vec<SystemResponse>>,
    username: Signal<String>,
}

impl ClientState {
    fn get_messages(&self) -> Vec<SystemResponse> {
        self.messages.read().to_vec()
    }
}

fn set_up_socket() -> (Coroutine<SystemRequest>, Signal<Vec<SystemResponse>>) {
    tracing::info!("Creating WebSocket connection...");
    let mut receiver_ws = use_signal(|| None); // will receive the websocket responses
    let ws_client = use_coroutine(move |mut rx: UnboundedReceiver<SystemRequest>| async move { // will send websocket requests received from the client into this coroutine
        let (mut sender, receiver) = WebSocket::open("ws://localhost:3000/message").unwrap().split(); //  split the websocket into a sender and receiver
        receiver_ws.set(Some(receiver)); // store the receiver in a signal so it can be used in another coroutine
        while let Some(msg) = rx.next().await { // wait for messages from the client
            tracing::debug!("Sending message: {:?}", msg);
            sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.unwrap(); // send the message to the server
        }
    });
    let mut message_list: Signal<Vec<SystemResponse>> = use_signal(|| vec![]); // will store the list of messages received from the server
    let _ = use_future(move || async move { // coroutine to handle incoming websocket messages
        if let Some(mut receiver) = receiver_ws.take() { // get the receiver from the signal
            while let Some(msg) = receiver.next().await { // wait for messages from the server
                if let Ok(msg) = msg { // if the message is valid
                    match msg { // match on the message type
                        Message::Text(content) => { // if it's a text message
                            let msg = serde_json::from_str(&content).unwrap();
                            tracing::debug!("Received message: {:?}", msg);
                            message_list.write().push(msg);
                        },
                        _ => () // ignore other message types
                    }
                }
            }
        }
    });
    (ws_client, message_list)
}