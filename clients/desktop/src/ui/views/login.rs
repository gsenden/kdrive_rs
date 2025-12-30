use common::domain::text_keys::TextKeys::{AuthenticateBtn, CopyLinkToBrowser, CopyText};
use common::ports::i18n_driven_port::I18nDrivenPort;
use dioxus::prelude::*;
use crate::ui::components::TitleBanner;

#[component]
pub fn Login<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort, url: String) -> Element {
    let url_for_browser = url.clone();
    let url_for_clipboard = url.clone();
    let mut clipboard_handle = use_signal(|| arboard::Clipboard::new().ok());

    rsx!(
        div {
            class: "min-h-screen flex flex-col items-center bg-[#0f1116] p-8 text-white",

            div {
                class: "pt-[15vh] flex flex-col items-center gap-y-10 w-full max-w-xl",

                TitleBanner { i18n: i18n.clone() },

                button {
                    class: "ml-10 mt-1 px-8 py-2 bg-blue-600 hover:bg-blue-500 text-white font-bold text-lg rounded-xl
                            transition-all duration-200 transform active:scale-95 shadow-lg shadow-blue-900/20 cursor-pointer",
                    onclick: move |_| {
                        let _ = webbrowser::open(&url_for_browser);
                    },
                    "{i18n.t(AuthenticateBtn)}"
                }
            }

            div { class: "flex-grow" }

            div {
                class: "w-full",
                p {
                    class: "text-slate-500 text-sm mb-3 text-center px-4",
                    {i18n.t(CopyLinkToBrowser)}
                }

                div {
                    class: "flex sm:flex-row flex-wrap gap-2 bg-slate-900/80 p-2 rounded-lg border border-slate-800 items-center",

                    p {
                        class: "flex-1 min-w-0 text-blue-400/70 font-mono text-xs break-all",
                        "{url}"
                    }

                    button {
                        class: "shrink-0 px-2 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-bold rounded
                                transition-colors duration-200 flex items-center gap-2 border border-slate-700",
                        onclick: move |_| {
                            if let Some(cb) = clipboard_handle.write().as_mut() {
                                let _ = cb.set_text(url_for_clipboard.clone());
                            }
                        },
                        {i18n.t(CopyText)}
                    }
                }
            }
        }
    )
}