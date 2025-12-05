use crate::domain::view::View;
use crate::ports::driven::server_driven_port::ServerDrivenPort;


pub struct Client<ServerPort>
where ServerPort: ServerDrivenPort {
    server_driven_port: ServerPort,
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
    use crate::domain::client::Client;
    use crate::domain::errors::ClientError;
    use crate::domain::test_helpers::fake_server_adapter::FakeServerAdapter;
    use crate::domain::view::View;
    use crate::Route::Home;

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
        // Given a client
        let mut server_adapter = FakeServerAdapter::new(true);
        let error = ClientError::ConnectionFailed("test error".to_string());
        server_adapter.set_error(error.clone());
        let client = Client::new(server_adapter);

        // When the page for the root route is requested
        let view = client.get_root_view().await;

        // Then the returned view is the Home View
        assert_eq!(view, View::Error(error.clone()));
    }
}

