use futures_util::stream;
use std::future;
use std::time::Duration;
use common::domain::errors::ApplicationError;
use common::kdrive::ServerEvent;
use common::kdrive::server_event::Event;
use crate::domain::events::ServerEventStream;
use crate::ports::driven::server_driven_port::ServerDrivenPort;

pub const TEST_URL_RESPONSE: &str = "http://localhost:8080/test-url-response";

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub struct FakeServerAdapter {
    authenticated: bool,
    error: Option<ApplicationError>,
    delay: Option<Duration>,
    event: Option<ServerEvent>,
}

#[allow(dead_code)]
impl FakeServerAdapter {

    pub fn new(authenticated: bool) -> Self {
        FakeServerAdapter { authenticated, error: None, delay: None, event: None }
    }

    pub fn slow(delay: Duration) -> Self {
        FakeServerAdapter { authenticated: false, error: None, delay: Some(delay), event: None }
    }

    pub fn with_event(event: Event) -> Self {
        FakeServerAdapter {
            authenticated: false,
            error: None,
            delay: None,
            event: Some(ServerEvent { event: Some(event) })
        }
    }

    pub fn with_server_event(event: ServerEvent) -> Self {
        FakeServerAdapter {
            authenticated: false,
            error: None,
            delay: None,
            event: Some(event),
        }
    }

    pub fn set_error(&mut self, error: ApplicationError) {
        self.error = Some(error);
    }

}

impl ServerDrivenPort for FakeServerAdapter {
    async fn is_authenticated(&self) -> Result<bool, ApplicationError> {
        if let Some(delay) = self.delay {
            tokio::time::sleep(delay).await;
        }

        if let Some(error) = &self.error {
            return Err(error.clone());
        } else {
            Ok(self.authenticated)
        }
    }

    async fn start_initial_auth_flow(&self) -> Result<String, ApplicationError> {
        if let Some(error) = &self.error {
            Err(error.clone())
        } else {
            Ok(TEST_URL_RESPONSE.to_string())

        }
    }

    async fn continue_initial_auth_flow(&self) -> Result<(), ApplicationError> {
        Ok(())
    }

    fn subscribe_events(&self) -> impl Future<Output=Result<ServerEventStream, ApplicationError>> + Send {
        let events: Vec<Result<ServerEvent, ApplicationError>> = match &self.event {
            Some(e) => vec![Ok(e.clone())],
            None => vec![],
        };

        future::ready(Ok(
            Box::pin(stream::iter(events)) as ServerEventStream
        ))
    }
}