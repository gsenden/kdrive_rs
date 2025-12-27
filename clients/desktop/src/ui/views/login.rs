use common::domain::text_keys::TextKeys::{AuthenticateBtn, AuthenticateWithBrowserMessage, CopyLinkToBrowser, CopyText, KDriveLogoAlt, KDriveProductName};
use common::ports::i18n_driven_port::I18nDrivenPort;
use dioxus::prelude::*;

use crate::adapters::grpc_server_adapter::GrpcServerAdapter;
use crate::domain::client::Client;
use crate::domain::errors::ClientError;
use crate::ports::driven::server_driven_port::ServerDrivenPort;
use crate::ui::views::{ConnectingView, ErrorView};
use crate::ui::components::TitleBanner;

#[component]
pub fn Login<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort) -> Element {
    let client = use_context::<Client<GrpcServerAdapter>>();
    let auth_result = get_auth_result(client);

    rsx! {
        LoginView {
            i18n: i18n,
            auth_result: auth_result
        }
    }

}

pub fn get_auth_result<P>(client: Client<P>) -> Signal<Option<Result<String, ClientError>>>
where
    P: ServerDrivenPort + Clone + PartialEq + 'static,
{
    let auth_result =
        use_signal::<Option<Result<String, ClientError>>>(|| None);

    use_effect(move || {
        let client = client.clone();
        let mut auth_result = auth_result.clone();

        spawn(async move {
            let result = client.on_login_view_shown().await;
            auth_result.set(Some(result));
        });
    });

    auth_result
}


#[component]
fn LoginView<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort, auth_result: Signal<Option<Result<String, ClientError>>>) -> Element {

    let content = match auth_result() {
        None => rsx! { ConnectingView { i18n: i18n.clone() } },
        Some(Ok(url)) => login_view_content(&i18n, url.clone()),
        Some(Err(err)) => rsx! { ErrorView {error: err.clone(), i18n: i18n.clone()} },
    };

    rsx! { {content} }
}
const KDRIVE_LOGO: Asset = asset!("/assets/kdrive.svg");

fn login_view_content<I18nPort: I18nDrivenPort + 'static>(i18n: &I18nPort, url: String) -> Element {
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

#[cfg(test)]
mod tests {
    use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
    use dioxus::prelude::*;
    use dioxus_ssr::render_element;
    use common::domain::text_keys::TextKeys::*;
    use common::ports::i18n_driven_port::I18nDrivenPort;
    use crate::domain::errors::ClientError;
    use crate::ui::views::login::LoginView;

    #[component]
    fn TestLoginView(initial: Option<Result<String, ClientError>>) -> Element {
        let i18n = I18nEmbeddedFtlAdapter::load();

        let auth_result = use_signal(|| initial);

        rsx! {
            LoginView {
                auth_result: auth_result,
                i18n: i18n
            }
        }
    }

    #[test]
    fn login_view_shows_authenticate_with_browser_message() {
        let i18n = I18nEmbeddedFtlAdapter::load();

        let html = render_element(rsx! {
            TestLoginView {
                initial: None::<Result<String, ClientError>>
            }
        });

        let expected = i18n.t(AuthenticateWithBrowserMessage);

        assert!(
            html.contains(&expected),
            "Expected auth message '{}', got HTML: {}",
            expected,
            html
        );
    }

    #[test]
    fn login_view_error() {
        // Given: A localized error created via our macro
        let error = crate::error!(TokenRequestFailed, Reason => "connection_refused");

        // When: Rendering the view via SSR
        let html = render_element(rsx! {
            TestLoginView { initial: Some(Err(error)) }
        });

        // Then: The UI should contain the translated string and the dynamic parameter.
        // This verifies that the UI "bridge" to our i18n system is working.
        assert!(html.contains("connection_refused"));
    }
}