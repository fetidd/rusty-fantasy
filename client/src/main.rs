use dioxus::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("assets/styles/app.css") }
        Home {}
    }
}

#[component]
fn Home() -> Element {
    let mut message_list: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut message_content = use_signal(|| String::new());
    let mut receiver_ws = use_signal(|| None);
    let mut name = use_signal(|| String::new());
    let mut has_name = use_signal(|| false);

    let chat_client = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        let (mut sender, receiver) = WebSocket::open("ws://localhost:3000/message").unwrap().split();
        receiver_ws.set(Some(receiver));
        while let Some(msg) = rx.next().await {
            let message = format!("{}: {}", name, msg);
            sender.send(Message::Text(message)).await.unwrap();
        }
    });

    let _ = use_future(move || async move {
        if let Some(mut receiver) = receiver_ws.take() {
            while let Some(msg) = receiver.next().await {
                if let Ok(msg) = msg {
                    match msg {
                        Message::Text(content) => {
                            message_list.write().push(content);
                        },
                        _ => ()
                    }
                }
            }
        }
    });
    rsx! {
        if !has_name() {
            div {
                class: "chat-container",
                div {
                    class: "chat input-name",
                    input {
                        r#type: "text",
                        placeholder: "Enter your name",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                    button {
                        onclick: move |_| has_name.set(true),
                        disabled: if name().trim() == "" {true},
                        "Join chat"
                    }
                }
            }
        } else {
            div { class: "chat-container",
                div { class: "chat",
                    div { class: "message-container",
                        {
                            message_list()
                                .iter()
                                .rev()
                                .map(|item| {
                                    let username = item.split(":").collect::<Vec<&str>>()[0];
                                    rsx! {
                                        p { class: "message-item", class: if username == name() { "user-message" }, "{item}" }
                                    }
                                })
                        }
                    }
                    div { class: "input-container",
                        input {
                            r#type: "text",
                            value: message_content,
                            placeholder: name,
                            oninput: move |e| message_content.set(e.value()),
                        }
                        button {
                            onclick: move |_| {
                                chat_client.send(message_content());
                                message_content.set(String::new());
                            },
                            disabled: if message_content().trim() == "" { true },
                            "Send"
                        }
                    }
                }
            }
        }
    }
}