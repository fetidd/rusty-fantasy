use dioxus::prelude::*;
use shared::{messaging::{SystemRequest, SystemResponse}};

use crate::ClientState;

#[component]
pub fn Chat() -> Element {
    let client = use_context::<ClientState>();
    let ws = use_context::<Coroutine<SystemRequest>>();
    let username = client.username.read().clone();
    let messages: Vec<_> = client.get_messages();
    let messages: Vec<_> = messages // only show chat and roll messages
        .iter()
        .rev()
        .filter(|item| matches!(item, SystemResponse::Chat {..} | SystemResponse::Roll {..}))
        .collect();

    let mut message_content = use_signal(|| String::new());

    let send_chat_message = { // send a chat message to the server
        let message_content = message_content.clone();
        let username = username.clone();
        move || {
            let msg = SystemRequest::Chat { username: username.clone(), role: "player".into(), content: message_content().clone() };
            ws.send(msg);
        }
    };

    let send_roll = { // send a roll request to the server
        let username = username.clone();
        let modifiers = client.current_modifiers.read().clone();
        move |_| {
            let msg = SystemRequest::Roll { username: username.clone(), modifiers: modifiers.clone() };
            ws.send(msg);
        }
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
                            SystemResponse::Roll {dice_values: (d1, d2), username, modifiers, total} => {
                                let modifiers = modifiers.to_string();
                                let content = if modifiers.is_empty() { format!("({d1}, {d2}) = {total}") } else { format!("({d1}, {d2}) ({modifiers}) = {total}") };
                                rsx! {
                                    div { class: "message roll",
                                        b { "{username} rolled: " }
                                        span { "{content}" }
                                    }
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
                        send_chat_message();
                        message_content.set(String::new());
                    },
                    disabled: if message_content().trim() == "" { true },
                    "Send"
                }
                button { onclick: send_roll, "Roll" }
            }
        }
    }
}