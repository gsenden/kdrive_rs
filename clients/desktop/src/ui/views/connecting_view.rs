use common::ports::i18n_driven_port::I18nDrivenPort;
use dioxus::prelude::*;
use common::domain::text_keys::TextKeys::ConnectingToServiceMessage;

#[component]
pub fn ConnectingView<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort) -> Element {
    rsx! {
        div { class: "loading",
            p { {i18n.t(ConnectingToServiceMessage)} }
        }
    }
}