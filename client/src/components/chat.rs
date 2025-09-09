use dioxus::prelude::*;
use shared::messaging::{SystemRequest, SystemResponse, TagMap};

use crate::ClientState;

#[component]
pub fn Chat() -> Element {
    let client = use_context::<ClientState>();
    let ws = use_context::<Coroutine<SystemRequest>>();
    let username = client.username.read().clone();
    let messages: Vec<_> = client.get_messages();
    let messages: Vec<_> = messages
        .iter()
        .rev()
        .filter(|item| matches!(item, SystemResponse::Chat {..} | SystemResponse::Roll {..}))
        .collect();

    let mut message_content = use_signal(|| String::new());
    let send_request = move |msg: SystemRequest| {
        ws.send(msg);
    };
    
    rsx! {
        div { class: "chat-container",
            div { class: "chat",
                div { 
                    class: "message-container",
                    {messages.iter().map(|msg| rsx! {
                        match msg {
                            SystemResponse::Chat {username, role, content} => rsx! {
                                div { class: "message",
                                    b { "{username} ({role}): " }
                                    span { "{content}" }
                                }
                            },
                            SystemResponse::Roll {dice_values: (d1, d2), username, tags, total} => rsx! {
                                div { class: "message roll",
                                    b { "{username} rolled: " }
                                    span { "({d1}, {d2}) + tags = {total}" }
                                }
                            },
                            _ => rsx! {}
                        }
                    }) }
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
                        let msg = SystemRequest::Chat { username: username.clone(), role: "player".into(), content: message_content().clone() };
                        send_request(msg);
                        message_content.set(String::new());
                    },
                    disabled: if message_content().trim() == "" { true },
                    "Send"
                }
                button {
                    onclick: move |_| {
                        let msg = SystemRequest::Roll { username: "derp".into(), tags: TagMap::default() };
                        send_request(msg);
                        message_content.set(String::new());
                    },
                    "Roll"
                }
            }
        }
    }
}