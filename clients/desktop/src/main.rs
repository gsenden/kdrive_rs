use dioxus::prelude::*;
use ui::views::{Blog, Home, Login, Navbar};
use domain::client::Client;
use adapters::grpc_server_adapter::GrpcServerAdapter;
use crate::domain::view::View;

mod adapters;
mod domain;
mod ports;
mod ui;

pub mod kdrive {
    tonic::include_proto!("kdrive");
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let client_resource = use_resource(|| async {
        let adapter = GrpcServerAdapter::connect().await?;
        Ok::<_, domain::errors::ClientError>(Client::new(adapter))
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        match &*client_resource.read() {
            Some(Ok(client)) => rsx! {
                AppWithClient { client: client.clone() }
            },
            Some(Err(e)) => rsx! {
                div { class: "error",
                    h1 { "Connection Error" }
                    p { "{e}" }
                }
            },
            None => rsx! {
                div { class: "loading",
                    p { "Connecting to server..." }
                }
            },
        }
    }
}

#[component]
fn AppWithClient(client: Client<GrpcServerAdapter>) -> Element {
    use_context_provider(|| client.clone());

    let view = use_resource(move || {
        let client = client.clone();
        async move {
            client.get_root_view().await
        }
    });

    rsx! {
        match &*view.read() {
            Some(View::Login) => rsx! { Login {} },
            Some(View::Home) => rsx! { Router::<Route> {} },
            Some(View::Error(e)) => rsx! { "Error: {e}" },
            None => rsx! { "Loading..." },
        }
    }
}