use std::time::Duration;
use common::application_error;
use common::domain::defaults::CONNECTION_TIMEOUT_SECONDS;
use common::domain::text_keys::TextKeys::ConnectionErrorMessage;
use crate::ports::driven::server_driven_port::ServerDrivenPort;
use crate::ports::driven::ui_driven_port::UIDrivenPort;
use common::kdrive::server_event::Event;
use futures_util::StreamExt;

pub struct UICore<Server, UI>
where
    Server: ServerDrivenPort,
    UI: UIDrivenPort,
{
    server: Server,
    ui: UI,
    timeout: Duration,
}

impl<Server, UI> UICore<Server, UI>
where
    Server: ServerDrivenPort,
    UI: UIDrivenPort,
{
    pub fn new(server: Server, ui: UI) -> Self {
        Self::with_timeout(server, ui, Duration::from_secs(CONNECTION_TIMEOUT_SECONDS))
    }

    pub fn with_timeout(server: Server, ui: UI, timeout: Duration) -> Self {
        Self { server, ui, timeout }
    }

    pub async fn run(&mut self) {
        // Subscribe to events
        let mut events = match self.server.subscribe_events().await {
            Ok(events) => events,
            Err(error) => {
                self.ui.show_error_view(error);
                return;
            }
        };

        self.start_up_view_logic().await;

        // Listen to events
        while let Some(Ok(server_event)) = events.next().await {
            if let Some(event) = server_event.event {
                self.handle_events(event);
            }
        }
    }

    fn handle_events(&mut self, event: Event) {
        match event {
            Event::AuthFlowCompleted(_) => {
                self.ui.show_home_view();
            }
            Event::Error(err) => {
                self.ui.show_error_view(err.into());
            }
        }
    }

    async fn start_up_view_logic(&mut self) {
        self.ui.show_loading_view();

        let result = tokio::time::timeout(
            self.timeout,
            self.server.is_authenticated()
        ).await;

        match result {
            Ok(Ok(false)) => self.auth_flow().await,
            Ok(Ok(true)) => self.ui.show_home_view(),
            Ok(Err(error)) => self.ui.show_error_view(error),
            Err(_connection_timeout) =>
                self.ui.show_error_view(application_error!(ConnectionErrorMessage)),
        }
    }

    async fn auth_flow(&mut self) {
        match self.server.start_initial_auth_flow().await {
            Ok(url) => {
                self.ui.show_login_view(url);

                let server = self.server.clone();
                tokio::spawn(async move {
                    let _ = server.continue_initial_auth_flow().await;
                });
            }
            Err(error) => {
                self.ui.show_error_view(error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;
    use crate::domain::test_helpers::fake_server_adapter::{FakeServerAdapter, TEST_URL_RESPONSE};
    use crate::domain::test_helpers::fake_ui_adapter::FakeUIAdapter;
    use common::application_error;
    use common::domain::defaults::CONNECTION_TIMEOUT_SECONDS;
    use common::domain::text_keys::TextKeys::ConnectionErrorMessage;
    use common::kdrive::server_event::Event;
    use common::kdrive::AuthFlowCompleted;

    #[tokio::test]
    async fn shows_error_view_when_server_returns_error() {
        // Given
        let mut server = FakeServerAdapter::new(false);
        server.set_error(application_error!(ConnectionErrorMessage));
        let ui = FakeUIAdapter::new();
        let mut core = UICore::new(server, ui.clone());

        // When
        core.run().await;

        // Then
        assert!(ui.error_view_was_shown());
    }

    #[tokio::test]
    async fn shows_login_view_when_not_authenticated() {
        // Given
        let server = FakeServerAdapter::new(false);
        let ui = FakeUIAdapter::new();
        let mut core = UICore::new(server, ui.clone());

        // When
        core.run().await;

        // Then
        assert!(ui.login_view_was_shown());
    }

    #[tokio::test]
    async fn shows_home_view_when_authenticated() {
        // Given
        let server = FakeServerAdapter::new(true);
        let ui = FakeUIAdapter::new();
        let mut core = UICore::new(server, ui.clone());

        // When
        core.run().await;

        // Then
        assert!(ui.home_view_was_shown());
    }

    #[tokio::test]
    async fn shows_loading_view_while_checking_authentication() {
        // Given
        let server = FakeServerAdapter::new(false);
        let ui = FakeUIAdapter::new();
        let mut core = UICore::new(server, ui.clone());

        // When
        core.run().await;

        // Then
        assert!(ui.loading_view_was_shown());
    }

    #[tokio::test]
    async fn shows_error_view_when_server_times_out() {
        // Given
        let server = FakeServerAdapter::slow(Duration::from_secs(CONNECTION_TIMEOUT_SECONDS + 1));
        let ui = FakeUIAdapter::new();
        let mut core = UICore::with_timeout(server, ui.clone(), Duration::from_secs(CONNECTION_TIMEOUT_SECONDS));

        // When
        core.run().await;

        // Then
        assert!(ui.error_view_was_shown());
    }

    #[tokio::test]
    async fn shows_login_url_when_auth_flow_starts() {
        let server = FakeServerAdapter::new(false);
        let ui = FakeUIAdapter::new();

        let mut core = UICore::new(server, ui.clone());
        core.run().await;

        assert_eq!(
            ui.login_url_shown(),
            Some(TEST_URL_RESPONSE.to_string())
        );
    }

    #[tokio::test]
    async fn shows_home_view_when_auth_flow_completed_event_received() {
        // Given
        let server = FakeServerAdapter::with_event(Event::AuthFlowCompleted(AuthFlowCompleted {}));

        let ui = FakeUIAdapter::new();
        let mut core = UICore::new(server, ui.clone());

        // When
        core.run().await;

        // Then
        assert!(ui.home_view_was_shown());
    }

    #[tokio::test]
    async fn shows_error_view_when_auth_flow_failed_event_received() {
        let error = application_error!(ConnectionErrorMessage);

        let server_event: common::kdrive::ServerEvent = error.into();

        let server = FakeServerAdapter::with_server_event(server_event);
        let ui = FakeUIAdapter::new();

        let mut core = UICore::new(server, ui.clone());
        core.run().await;

        assert!(ui.error_view_was_shown());
    }
}