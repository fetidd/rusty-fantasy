use dioxus::prelude::*;

use crate::ClientState;

#[component]
pub fn LoginScreen() -> Element {
    let mut client = use_context::<ClientState>();
    let mut name_input = use_signal(|| String::new());
    rsx! {
        div { class: "name-container",
            div { class: "name-input",
                h2 { "Enter your name to join the session" }
                input {
                    r#type: "text",
                    value: name_input.read().clone(),
                    placeholder: "Your name",
                    oninput: move |e| name_input.set(e.value()),
                }
                button {
                    onclick: move |_| {
                        tracing::debug!("Setting username to: {}", name_input.read());
                        if !name_input.read().trim().is_empty() {
                            client.username.set(name_input.read().clone());
                        }
                        tracing::debug!("Username set to: {}", client.username);
                    },
                    disabled: if name_input.read().trim() == "" { true } else { false },
                    "Join"
                }
            }
        }
    }
}
