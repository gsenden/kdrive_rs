use std::future::Future;
use futures_util::StreamExt;
use tonic::transport::Channel;
use std::time::Duration;
use tonic::Request;
use common::{
    domain::text_keys::TextKeys::ConnectionErrorMessage,
    domain::errors::*,
    application_error,
    domain::defaults::{CONNECTION_TIMEOUT_SECONDS, DEFAULT_SERVER_URL}

};
use common::kdrive::Empty;
use common::kdrive::kdrive_service_client::KdriveServiceClient;
use crate::{
    domain::events::ServerEventStream,
    ports::driven::server_driven_port::ServerDrivenPort
};


#[derive(Clone)]
pub struct GrpcServerAdapter {
    client: KdriveServiceClient<Channel>,
}

impl PartialEq for GrpcServerAdapter {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl GrpcServerAdapter {
    async fn connect_with_url(url: &'static str) -> Result<Self, ApplicationError> {
        let channel = Channel::from_static(url)
            .connect_timeout(Duration::from_secs(CONNECTION_TIMEOUT_SECONDS))
            .connect()
            .await
            .map_err(|e| {
                application_error!(ConnectionErrorMessage, e.to_string())
            })?;

        Ok(Self {
            client: KdriveServiceClient::new(channel),
        })
    }

    pub async fn connect() -> Result<Self, ApplicationError> {
        Self::connect_with_url(DEFAULT_SERVER_URL).await
    }
}

impl ServerDrivenPort for GrpcServerAdapter {
    fn is_authenticated(&self) -> impl Future<Output = Result<bool, ApplicationError>> + Send {
        let mut client = self.client.clone();
        async move {
            let response = client
                .is_authenticated(Empty {})
                .await
                .map_err(|status| ApplicationError::from(status))?;

            Ok(response.into_inner().is_authenticated)
        }
    }

    fn start_initial_auth_flow(&self) -> impl Future<Output=Result<String, ApplicationError>> + Send {
        let mut client = self.client.clone();
        async move {
            let response = client
                .start_initial_auth_flow(Empty {})
                .await
                .map_err(|status| ApplicationError::from(status) )?;

            Ok(response.into_inner().auth_url)
        }
    }

    fn continue_initial_auth_flow(&self) -> impl Future<Output=Result<(), ApplicationError>> + Send {
        let mut client = self.client.clone();
        async move {
            client
                .continue_initial_auth_flow(Empty {})
                .await
                .map_err(|status| ApplicationError::from(status))?;

            Ok(())
        }
    }

    fn subscribe_events(&self)
        -> impl Future<Output=Result<ServerEventStream, ApplicationError>> + Send
    {
        let mut client = self.client.clone();

        async move {
            let response = client
                .subscribe_events(Request::new(Empty {}))
                .await
                .map_err(ApplicationError::from)?;

            let stream = response
                .into_inner()
                .map(|item| item.map_err(ApplicationError::from));

            Ok(Box::pin(stream) as ServerEventStream)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
    use tokio_stream::StreamExt;
    use crate::adapters::test_helpers::fake_kdrive_service::start_test_server;
    use common::kdrive::server_event::Event as ServerEventKind;

    #[tokio::test]
    async fn connect_fails_within_reasonable_time_when_target_is_unreachable() {
        let unreachable_url = "http://192.0.2.1:50051";

        let start = std::time::Instant::now();
        let result = GrpcServerAdapter::connect_with_url(unreachable_url).await;

        let elapsed = start.elapsed();

        assert!(result.is_err());
        assert!(
            elapsed < Duration::from_secs(CONNECTION_TIMEOUT_SECONDS + 3),
            "connect took too long: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn client_receives_auth_flow_completed_event_from_server() {
        // Given a running gRPC server emitting AuthFlowCompleted
        let (server_url, _handle) = start_test_server().await;

        // And a gRPC server adapter connected to it
        let adapter =
            GrpcServerAdapter::connect_with_url(Box::leak(server_url.into_boxed_str()))
                .await
                .unwrap();

        // When the client subscribes to server events
        let mut events = adapter.subscribe_events().await.unwrap();

        // Then the first event must be AuthFlowCompleted
        let event = events.next().await.unwrap().unwrap();

        assert!(matches!(
            event.event,
            Some(ServerEventKind::AuthFlowCompleted(_))
        ));
    }
}