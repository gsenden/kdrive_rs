use std::time::Duration;
use common::application_error;
use common::domain::defaults::CONNECTION_TIMEOUT_SECONDS;
use common::domain::errors::ApplicationError;
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

    pub async fn start(&self) {
        self.ui.show_loading_view();

        let result = tokio::time::timeout(
            self.timeout,
            self.server.is_authenticated()
        ).await;

        match result {
            Ok(Ok(false)) => self.ui.show_login_view(),
            Ok(Ok(true)) => self.ui.show_home_view(),
            Ok(Err(error)) => self.ui.show_error_view(error),
            Err(_connection_timeout) => self.ui.show_error_view(application_error!(ConnectionErrorMessage)),
        }

        // Event loop
        if let Ok(mut events) = self.server.subscribe_events().await {
            while let Some(Ok(server_event)) = events.next().await {
                if let Some(event) = server_event.event {
                    match event {
                        Event::AuthFlowCompleted(_) => self.ui.show_home_view(),
                        Event::Error(err) => self.ui.show_error_view(err.into()),
                    }
                }
            }
        }
    }

    pub async fn on_login_view_shown(&self) -> Result<String, ApplicationError> {
        let url = self.server.start_initial_auth_flow().await?;
        self.server.continue_initial_auth_flow().await?;
        Ok(url)
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
        let core = UICore::new(server, ui.clone());

        // When
        core.start().await;

        // Then
        assert!(ui.error_view_was_shown());
    }

    #[tokio::test]
    async fn shows_login_view_when_not_authenticated() {
        // Given
        let server = FakeServerAdapter::new(false);
        let ui = FakeUIAdapter::new();
        let core = UICore::new(server, ui.clone());

        // When
        core.start().await;

        // Then
        assert!(ui.login_view_was_shown());
    }

    #[tokio::test]
    async fn shows_home_view_when_authenticated() {
        // Given
        let server = FakeServerAdapter::new(true);
        let ui = FakeUIAdapter::new();
        let core = UICore::new(server, ui.clone());

        // When
        core.start().await;

        // Then
        assert!(ui.home_view_was_shown());
    }

    #[tokio::test]
    async fn shows_loading_view_while_checking_authentication() {
        // Given
        let server = FakeServerAdapter::new(false);
        let ui = FakeUIAdapter::new();
        let core = UICore::new(server, ui.clone());

        // When
        core.start().await;

        // Then
        assert!(ui.loading_view_was_shown());
    }

    #[tokio::test]
    async fn shows_error_view_when_server_times_out() {
        // Given
        let server = FakeServerAdapter::slow(Duration::from_secs(CONNECTION_TIMEOUT_SECONDS + 1));
        let ui = FakeUIAdapter::new();
        let core = UICore::with_timeout(server, ui.clone(), Duration::from_secs(CONNECTION_TIMEOUT_SECONDS));

        // When
        core.start().await;

        // Then
        assert!(ui.error_view_was_shown());
    }

    #[tokio::test]
    async fn starts_auth_flow_when_login_view_shown() {
        // Given
        let server = FakeServerAdapter::new(false);
        let ui = FakeUIAdapter::new();
        let core = UICore::new(server, ui);

        // When
        let url = core.on_login_view_shown().await.unwrap();

        // Then
        assert_eq!(url, TEST_URL_RESPONSE);
    }

    #[tokio::test]
    async fn shows_home_view_when_auth_flow_completed_event_received() {
        // Given
        let server = FakeServerAdapter::with_event(Event::AuthFlowCompleted(AuthFlowCompleted {}));

        let ui = FakeUIAdapter::new();
        let core = UICore::new(server, ui.clone());

        // When
        core.start().await;

        // Then
        assert!(ui.home_view_was_shown());
    }
}