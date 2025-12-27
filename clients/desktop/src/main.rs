use dioxus::prelude::*;
use dioxus::desktop::use_window;
use dioxus::desktop::tao::platform::windows::IconExtWindows;
use dioxus::desktop::tao::window::Icon;
use ui::views::{Blog, Home, Login, Navbar};
use domain::client::Client;
use adapters::grpc_server_adapter::GrpcServerAdapter;
use crate::domain::view::View;
use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
use common::domain::errors::ApplicationError;
use common::ports::i18n_driven_port::I18nDrivenPort;
use common::domain::text_keys::TextKeys;
#[cfg(target_os = "windows")]
use common::domain::text_keys::TextKeys::FailedToLoadWindowsIcon;
#[cfg(target_os = "linux")]
use common::domain::text_keys::TextKeys::FailedToLoadLinuxIcon;
use common::domain::text_keys::TextKeys::WindowTitle;
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

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const SUISSE_FONT: Asset = asset!("/assets/suisse-intl-400-normal.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let window = use_window();

    let i18n = use_hook(|| {
        I18nEmbeddedFtlAdapter::load()
    });

    let i18n_for_window = i18n.clone();
    use_effect(move || {
        window.set_title(&i18n_for_window.t(WindowTitle));
        window.set_window_icon(Some(load_icon(i18n_for_window.clone())));
    });

    let client_resource = use_resource(|| create_client());

    let font_css = use_memo(|| {
        format!(":root {{ --suisse-font-url: url({}); }}", SUISSE_FONT)
    });

    rsx! {
        document::Style {
            r#type: "text/css",
            {font_css}
        }

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
            Some(View::Error(error)) => {
                let msg = error.translate(&i18n);
                rsx! { "Error: {msg}" }
            },
            None => rsx! { "Loading..." },
        }
    }
}

async fn create_client() -> Result<Client<GrpcServerAdapter>, ApplicationError> {
    let grpc_adapter = GrpcServerAdapter::connect().await?;
    Ok(Client::new(grpc_adapter))
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

