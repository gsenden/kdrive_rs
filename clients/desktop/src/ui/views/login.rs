use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
use common::domain::text_keys::TextKeys::{AuthenticateWithBrowserMessage, ErrorMsg, FlowNotStarted};
use common::ports::i18n_driven_port::I18nDrivenPort;
use dioxus::prelude::*;

use crate::adapters::grpc_server_adapter::GrpcServerAdapter;
use crate::domain::client::Client;
use crate::domain::errors::{translate_error, ClientError};
use crate::ports::driven::server_driven_port::ServerDrivenPort;
use crate::ui::text::{
    AUTHENTICATE_WITH_BROWSER_MESSAGE,
    AUTHENTICATION_ERROR_PREFIX,
};
use crate::ui::utils::Pipe;

#[component]
pub fn Login() -> Element {
    rsx! {
    {
        use_context::<Client<GrpcServerAdapter>>()
            .pipe(get_auth_result)
            .pipe(|auth_result| rsx! {
                LoginView { auth_result }
            })
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
fn LoginView(auth_result: Signal<Option<Result<String, ClientError>>>) -> Element {
    let i18n_signal = use_context::<Signal<I18nEmbeddedFtlAdapter>>();
    let i18n = i18n_signal.read();

    let content = match auth_result() {
        None => rsx!(p { {i18n.t(AuthenticateWithBrowserMessage)} }),
        Some(Ok(url)) => rsx!(
            div {
                p { { i18n.t(AuthenticateWithBrowserMessage) } }
                p { {url} }
            }
        ),
        Some(Err(err)) => {
            let error_msg = translate_error(&err, &i18n);

            rsx!(
                p { class: "error", {error_msg} }
            )
        },
    };

    rsx! { {content} }
}

#[cfg(test)]
mod tests {
    use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
    use common::ports::i18n_driven_port::I18nDrivenPort;
    use dioxus::prelude::*;
    use dioxus_ssr::render_element;

    use crate::domain::errors::ClientError;
    use crate::ui::text::{AUTHENTICATE_WITH_BROWSER_MESSAGE, AUTHENTICATION_ERROR_PREFIX};
    use crate::ui::views::login::LoginView;

    #[component]
    fn TestLoginView(initial: Option<Result<String, ClientError>>) -> Element {
        // Zorg dat i18n beschikbaar is voor SSR tests
        let i18n = use_signal(|| I18nEmbeddedFtlAdapter::load().unwrap());
        use_context_provider(|| i18n);

        let auth_result = use_signal(|| initial);
        rsx! { LoginView { auth_result } }
    }

    #[test]
    fn login_view_shows_loading_message() {
        let html = render_element(rsx! {
            TestLoginView {
                initial: None::<Result<String, ClientError>>
            }
        });

        assert!(html.contains(AUTHENTICATE_WITH_BROWSER_MESSAGE));
    }

    #[component]
    fn TestWrapper(initial: Option<Result<String, ClientError>>) -> Element {
        // Initialize i18n signal for the UI test context
        let i18n = use_signal(|| I18nEmbeddedFtlAdapter::load().unwrap());
        use_context_provider(|| i18n);

        let auth_result = use_signal(|| initial);
        rsx! { LoginView { auth_result } }
    }

    #[test]
    fn login_view_error() {
        // Given: A localized error created via our macro
        let error = crate::error!(TokenRequestFailed, Reason => "connection_refused");

        // When: Rendering the view via SSR
        let html = render_element(rsx! {
            TestWrapper { initial: Some(Err(error)) }
        });

        // Then: The UI should contain the translated string and the dynamic parameter.
        // This verifies that the UI "bridge" to our i18n system is working.
        assert!(html.contains("connection_refused"));
    }
}