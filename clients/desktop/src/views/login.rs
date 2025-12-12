use dioxus::prelude::*;
use crate::adapters::grpc_server_adapter::GrpcServerAdapter;
use crate::ports::driven::server_driven_port::ServerDrivenPort;
use crate::domain::client::Client;
const AUTHENTICATE_WITH_BROWSER_MESSAGE: &str = "Please authenticate using the browser.";
const AUTHENTICATION_ERROR_PREFIX: &str = "Error starting the authentication:";

#[component]
pub fn Login() -> Element {
    let client = use_context::<Client<GrpcServerAdapter>>();
    rsx! {
        LoginWithClient { client }
    }
}

#[component]
fn LoginWithClient<P>(client: Client<P>) -> Element
where
    P: ServerDrivenPort + Clone + PartialEq + 'static,
{
    let url = use_resource(move || {
        let client = client.clone();
        async move {
            client.on_login_view_shown().await
        }
    });

    let content = match url() {
        None => rsx!(p { {AUTHENTICATE_WITH_BROWSER_MESSAGE} }),
        Some(Ok(login_url)) => rsx!(
            div {
                p { {AUTHENTICATE_WITH_BROWSER_MESSAGE} }
                p { {login_url} }
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
    use dioxus::core::Mutations;
    use dioxus::prelude::*;
    use crate::domain::client::Client;
    use crate::domain::test_helpers::fake_server_adapter::FakeServerAdapter;
    use crate::views::login::{LoginWithClient, LoginWithClientProps, AUTHENTICATE_WITH_BROWSER_MESSAGE};
    use dioxus_ssr::render;

    #[tokio::test]
    async fn login_shows_loading_message_initially(){
        // Given a client
        let client = Client::new(FakeServerAdapter::new(false));

        // and a virtual dom
        let mut dom = VirtualDom::new_with_props(
            LoginWithClient,
            LoginWithClientProps { client },
        );

        // When the component is rendered
        let mut mutations = Mutations::default();
        dom.rebuild(&mut mutations);
        let html = render(&dom);

        // Then the loading message is shown
        assert!(html.contains(AUTHENTICATE_WITH_BROWSER_MESSAGE));
    }
}