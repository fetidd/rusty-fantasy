use dioxus::prelude::*;

use crate::ClientState;

#[component]
pub fn Client() -> Element {
    let client = use_context::<ClientState>();
    rsx! {
        document::Stylesheet { href: asset!("assets/styles/client.css") }
        div { class: "client-container",}
        if client.username.read().is_empty() {
            super::login_screen::LoginScreen {}
        } else {
            super::chat::Chat {}
        }
    }
}