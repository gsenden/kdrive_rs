use common::ports::i18n_driven_port::I18nDrivenPort;
use dioxus::prelude::*;
use common::domain::text_keys::TextKeys::ConnectingToServiceMessage;
use crate::ui::components::TitleBanner;

#[component]
pub fn ConnectingView<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort) -> Element {
    rsx! {
        div {
            class: "min-h-screen flex flex-col items-center bg-[#0f1116] p-8 text-white",

            div {
                class: "pt-[15vh] flex flex-col items-center gap-y-10 w-full max-w-xl",
                TitleBanner { i18n: i18n.clone() },
            }

            div {
                class: "mt-8 flex flex-col items-center text-center max-w-xl",
                h2 {
                    class: "text-2xl loading",
                    {i18n.t(ConnectingToServiceMessage)}
                }

            }
        }

    }
}