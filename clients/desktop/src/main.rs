use dioxus::prelude::*;
use dioxus::desktop::use_window;
use dioxus::desktop::tao::window::Icon;
#[cfg(target_os = "windows")]
use dioxus::desktop::tao::platform::windows::IconExtWindows;
use adapters::grpc_server_adapter::GrpcServerAdapter;
use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
use common::ports::i18n_driven_port::I18nDrivenPort;
use common::domain::text_keys::TextKeys;
#[cfg(target_os = "windows")]
use common::domain::text_keys::TextKeys::FailedToLoadWindowsIcon;
#[cfg(target_os = "linux")]
use common::domain::text_keys::TextKeys::FailedToLoadLinuxIcon;
use common::domain::text_keys::TextKeys::WindowTitle;
use crate::adapters::dioxus_adapter::DioxusAdapter;
use crate::domain::ui_core::UICore;
use crate::ports::driven::ui_driven_port::UIDrivenPort;
use crate::ui::views::ConnectingView;

mod adapters;
mod domain;
mod ports;
mod ui;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const SUISSE_FONT: Asset = asset!("/assets/suisse-intl-400-normal.woff2");

fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let window = use_window();
    let i18n = use_hook(|| I18nEmbeddedFtlAdapter::load());

    let i18n_for_signal = i18n.clone();
    let element_signal =
        use_signal(move || rsx! { ConnectingView { i18n: i18n_for_signal.clone() } });


    let i18n_for_window = i18n.clone();
    use_effect(move || {
        window.set_title(&i18n_for_window.t(WindowTitle));
        window.set_window_icon(Some(load_icon(i18n_for_window.clone())));
    });

    let dioxus_adapter =
        use_hook(|| DioxusAdapter::new(element_signal, i18n.clone()));

    // Start UICore
    use_future(move || {
        let adapter_for_core = dioxus_adapter.clone();
        async move {
            if let Ok(grpc_adapter) = GrpcServerAdapter::connect().await {
                let mut core =
                    UICore::new(grpc_adapter, adapter_for_core);
                core.run().await;
            }
        }
    });

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

        {element_signal()}
    }
}


fn load_icon<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort) -> Icon {
    #[cfg(target_os = "windows")]
    {
        Icon::from_path("assets/kdrive_icon.ico", None).expect(i18n.t(FailedToLoadWindowsIcon).as_str())
    }

    #[cfg(target_os = "linux")]
    {
        let icon_bytes = include_bytes!("../assets/kdrive_icon.png");
        let image = image::load_from_memory(icon_bytes)
            .expect(i18n.t(FailedToLoadLinuxIcon).as_str())
            .into_rgba8();
        let (width, height) = image.dimensions();
        Icon::from_rgba(image.into_raw(), width, height)
            .expect(i18n.t(FailedToLoadLinuxIcon).as_str())
    }

    #[cfg(target_os = "macos")]
    {
        Icon::from_path("assets/kdrive_icon.icns", None).expect(i18n.t(FailedToLoadWindowsIcon).as_str())
    }
}

