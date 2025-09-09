use dioxus::prelude::*;
use shared::messaging::{SystemRequest, SystemResponse};

use crate::ClientState;

#[component]
pub fn Chat() -> Element {
    let client = use_context::<ClientState>();
    let mut message_content = use_signal(|| String::new());
    
    rsx! {
        div { class: "chat-container",
            div { class: "chat",
                div { 
                    class: "message-container",
                    {
                        client.get_messages()
                        .iter()
                        .rev()
                        .filter_map(|item| {
                            if let SystemResponse::Chat { username, role: _, content } = item {
                                let display_name = if *username == *client.username.read() { "You" } else { username };
                                Some(rsx! {
                                    p { class: "message-item chat", "{display_name}: {content}" }
                                })
                            } else if let SystemResponse::Roll { dice_values: (d1, d2), username, tags: _ } = item {
                                let display_name = if *username == *client.username.read() { "You" } else { username };
                                Some(rsx! {
                                    p { class: "message-item roll", "{display_name} rolled a {d1 + d2}" }
                                })
                            } else {
                                None
                            }
                        })
                    }
                }
            }
            div { class: "input-container",
                input {
                    r#type: "text",
                    value: message_content,
                    oninput: move |e| message_content.set(e.value()),
                }
                button {
                    onclick: move |_| {
                        let msg = SystemRequest::Chat { username: client.username.read().clone(), role: "player".into(), content: message_content().clone() };
                        client.send(msg);
                        message_content.set(String::new());
                    },
                    disabled: if message_content().trim() == "" { true },
                    "Send"
                }
            }
        }
    }
}