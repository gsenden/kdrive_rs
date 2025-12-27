use common::domain::errors::ApplicationError;
use crate::domain::view::View;
use crate::ports::driven::server_driven_port::ServerDrivenPort;

#[derive(PartialEq, Clone)]
pub struct Client<ServerPort>
where ServerPort: ServerDrivenPort {
    server_driven_port: ServerPort,
}

impl<ServerPort> Client<ServerPort>
where
    ServerPort: ServerDrivenPort
{
    pub(crate) async fn on_login_view_shown(&self) -> Result<String, ApplicationError> {
       self.server_driven_port.start_initial_auth_flow().await
    }
}

impl<ServerPort> Client<ServerPort>
where ServerPort: ServerDrivenPort {
    pub fn new(server_driven_port: ServerPort) -> Self {
        Client {server_driven_port}
    }

    pub async fn get_root_view(&self) -> View {
        match self.server_driven_port.is_authenticated().await {
            Ok(true) => View::Home,
            Ok(false) => View::Login,
            Err(error) => View::Error(error)

        }
    }
}

#[cfg(test)]
mod tests {
    use common::application_error;
    use common::domain::text_keys::TextKeys::TokenRequestFailed;
    use crate::domain::client::Client;
    use crate::domain::test_helpers::fake_server_adapter::{FakeServerAdapter, TEST_URL_RESPONSE};
    use crate::domain::view::View;


    #[tokio::test]
    async fn if_the_server_is_authenticated_then_the_home_page_is_shown() {
        // Given a client
        let server_adapter = FakeServerAdapter::new(true);
        let client = Client::new(server_adapter);

        // When the page for the root route is requested
        let view = client.get_root_view().await;

        // Then the returned view is the Home View
        assert_eq!(view, View::Home);
    }

    #[tokio::test]
    async fn if_the_server_is_not_authenticated_then_the_login_page_is_shown() {
        // Given a client
        let server_adapter = FakeServerAdapter::new(false);
        let client = Client::new(server_adapter);

        // When the page for the root route is requested
        let view = client.get_root_view().await;

        // Then the returned view is the Home View
        assert_eq!(view, View::Login);
    }

    #[tokio::test]
    async fn if_the_connection_to_the_server_has_an_error_then_the_error_page_is_shown() {
        // Given: A server adapter configured to return a specific Localized error
        let mut server_adapter = FakeServerAdapter::new(true);

        // We create the expected error using the macro.
        // This tests data structures rather than translated strings.
        let expected_error = application_error!(TokenRequestFailed, "test error");
   
        server_adapter.set_error(expected_error.clone());
        let client = Client::new(server_adapter);

        // When: The root view is requested
        let view = client.get_root_view().await;

        // Then: The view must be an Error variant containing exactly the data we provided.
        // This ensures the logic propagates the correct key and parameters.
        assert_eq!(view, View::Error(expected_error));
    }

    #[tokio::test]
    async fn client_starts_authentication_on_login_view_shown_and_returns_auth_url() {
        // Given a client
        let server_adapter = FakeServerAdapter::new(false);
        let client = Client::new(server_adapter);

        // When the client is notified that the login view is shown
        let url = client.on_login_view_shown().await.unwrap();

        // Then the url is returned
        assert_eq!(url, TEST_URL_RESPONSE);
    }
}

