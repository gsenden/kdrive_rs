use dioxus::prelude::*;

use crate::adapters::grpc_server_adapter::GrpcServerAdapter;
use crate::domain::client::Client;
use crate::domain::errors::ClientError;
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

    let content = match auth_result() {
        None => rsx!(p { {AUTHENTICATE_WITH_BROWSER_MESSAGE} }),
        Some(Ok(url)) => rsx!(
            div {
                p { {AUTHENTICATE_WITH_BROWSER_MESSAGE} }
                p { {url} }
            }
        ),
        Some(Err(err)) => rsx!(
            p { class: "error", "{AUTHENTICATION_ERROR_PREFIX} {err}" }
        ),
    };

    rsx! { {content} }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render_element;

    use crate::domain::errors::ClientError;
    use crate::ui::text::{AUTHENTICATE_WITH_BROWSER_MESSAGE, AUTHENTICATION_ERROR_PREFIX};
    use crate::ui::views::login::LoginView;

    #[component]
    fn TestLoginView(initial: Option<Result<String, ClientError>>) -> Element {
        let auth_result = use_signal(|| initial);

        rsx! {
            LoginView { auth_result }
        }
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

    #[test]
    fn login_view_shows_the_url() {
        let html = render_element(rsx! {
        TestLoginView {
            initial: Some(Ok("http://test.url".to_string()))
        }
    });

        assert!(html.contains("http://test.url"));
    }

    #[test]
    fn login_view_error() {
        let html = render_element(rsx! {
        TestLoginView {
            initial: Some(Err(
                ClientError::ConnectionFailed("boom".into())
            ))
        }
    });

        assert!(html.contains(AUTHENTICATION_ERROR_PREFIX));
    }
}