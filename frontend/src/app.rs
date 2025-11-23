#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::tauri_commands::TauriCommands;

static CSS: Asset = asset!("/assets/styles.css");
static TAURI_ICON: Asset = asset!("/assets/tauri.svg");
static DIOXUS_ICON: Asset = asset!("/assets/dioxus.png");

pub fn App() -> Element {
    let mut name = use_signal(|| String::new());
    let mut greet_msg = use_signal(|| String::new());

    let greet = move |_: FormEvent| async move {
        if name.read().is_empty() {
            return;
        }

        let name = name.read();

        match TauriCommands::greet(&name).await {
            Ok(msg) => greet_msg.set(msg),
            Err(_) => return // TODO: handle error
        }
    };

    rsx! {
        link { rel: "stylesheet", href: CSS }
        main {
            class: "container",
            h1 { "Welcome to Tauri + Dioxus" }

            div {
                class: "row",
                a {
                    href: "https://tauri.app",
                    target: "_blank",
                    img {
                        src: TAURI_ICON,
                        class: "logo tauri",
                         alt: "Tauri logo"
                    }
                }
                a {
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    img {
                        src: DIOXUS_ICON,
                        class: "logo dioxus",
                        alt: "Dioxus logo"
                    }
                }
            }
            p { "Click on the Tauri and Dioxus logos to learn more." }

            form {
                class: "row",
                onsubmit: greet,
                input {
                    id: "greet-input",
                    placeholder: "Enter a name...",
                    value: "{name}",
                    oninput: move |event| name.set(event.value())
                }
                button { r#type: "submit", "Greet" }
            }
            p { "{greet_msg}" }
        }
    }
}