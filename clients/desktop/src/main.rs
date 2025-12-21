use dioxus::launch;
use dioxus::prelude::*;
use dioxus::desktop::{use_window, Config, WindowBuilder};
use dioxus::desktop::tao::platform::windows::IconExtWindows;
use dioxus::desktop::tao::window::Icon;
use ui::views::{Blog, Home, Login, Navbar};
use domain::client::Client;
use adapters::grpc_server_adapter::GrpcServerAdapter;
use crate::domain::view::View;
use crate::domain::errors::{translate_error, ClientError};

// I18n imports
use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
use common::ports::i18n_driven_port::I18nDrivenPort;
use common::domain::text_keys::TextKeys;
use common::domain::text_keys::TextKeys::{FailedToLoadLinuxIcon, FailedToLoadWindowsIcon, WindowTitle};
use crate::ui::views::{ConnectingView, ErrorView};

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



async fn create_client() -> Result<Client<GrpcServerAdapter>, ClientError> {
    let grpc_adapter = GrpcServerAdapter::connect().await?;
    Ok(Client::new(grpc_adapter))
}

#[component]
fn App() -> Element {
    

    let i18n = use_hook(|| {
        I18nEmbeddedFtlAdapter::load().expect("Failed to load localizations")
    });
    
    set_icon_and_window_title(i18n.clone());

    let client_resource = use_resource(|| create_client());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        match &*client_resource.read() {
            None => rsx! {
                ConnectingView { i18n: i18n.clone() }
            },
            Some(Ok(client)) => rsx! {
                AppWithClient { client: client.clone(), i18n: i18n.clone() }
            },
            Some(Err(e)) => rsx! {
                ErrorView { error: e.clone(), i18n: i18n.clone() }
            },
        }
    }
}

#[component]
fn AppWithClient<I18nPort: I18nDrivenPort + 'static>(client: Client<GrpcServerAdapter>, i18n: I18nPort) -> Element {
    use_context_provider(|| client.clone());

    let view = use_resource(move || {
        let client = client.clone();
        async move {
            client.get_root_view().await
        }
    });

    rsx! {
        match &*view.read() {
            Some(View::Login) => rsx! { Login { i18n } },
            Some(View::Home) => rsx! { Router::<Route> {} },
            Some(View::Error(e)) => {
                let msg = translate_error(e, &i18n);
                rsx! { "Error: {msg}" }
            },
            None => rsx! { "Loading..." },
        }
    }
}

fn set_icon_and_window_title<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort) {
    let window = use_window();

    use_effect(move || {
        window.set_title(i18n.t(WindowTitle).as_str());
        window.set_window_icon(Some(load_icon(i18n.clone())));
    });
}

fn load_icon<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort) -> Icon {
    #[cfg(target_os = "windows")]
    {
        Icon::from_path("assets/kdrive_icon.ico", None).expect(i18n.t(FailedToLoadWindowsIcon).as_str())
    }

    #[cfg(target_os = "linux")]
    {
        Icon::from_path("assets/kdrive_icon.png", None).expect(i18n.t(FailedToLoadLinuxIcon).as_str())
    }

    #[cfg(target_os = "macos")]
    {
        Icon::from_path("assets/kdrive_icon.icns", None).expect(i18n.t(FailedToLoadWindowsIcon).as_str())
    }
}

