use common::ports::i18n_driven_port::I18nDrivenPort;
use dioxus::prelude::*;
use common::domain::errors::ApplicationError;
use common::domain::text_keys::TextKeys::ConnectionError;
use crate::ui::components::TitleBanner;

#[component]
pub fn ErrorView<I18nPort: I18nDrivenPort + 'static>(error: ApplicationError, i18n: I18nPort) -> Element {
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
                    class: "text-2xl text-yellow-400",

                    span {
                        class: "text-yellow-400 text-xl",
                        "⚠  "
                    }
                    {i18n.t(ConnectionError)}
                }

                p {
                    class: "mt-3 text-sm text-slate-400",
                    {error.translate(&i18n)}
                }
            }
        }
    }
}