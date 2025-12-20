use std::future::Future;
use common::domain::defaults::{CONNECTION_TIMEOUT_SECONDS, DEFAULT_SERVER_URL};
use tonic::transport::Channel;
use std::time::Duration;

use crate::domain::errors::ClientError;
use crate::error;
use crate::kdrive::kdrive_service_client::KdriveServiceClient;
use crate::kdrive::Empty;
use crate::ports::driven::server_driven_port::ServerDrivenPort;



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
    async fn connect_with_url(url: &'static str) -> Result<Self, ClientError> {
        let channel = Channel::from_static(url)
            .connect_timeout(Duration::from_secs(CONNECTION_TIMEOUT_SECONDS))
            .connect()
            .await
            .map_err(|e| {
                error!(ConnectionErrorMessage, Reason => e.to_string())
            })?;

        Ok(Self {
            client: KdriveServiceClient::new(channel),
        })
    }

    pub async fn connect() -> Result<Self, ClientError> {
        Self::connect_with_url(DEFAULT_SERVER_URL).await
    }
}

impl ServerDrivenPort for GrpcServerAdapter {
    fn is_authenticated(&self) -> impl Future<Output = Result<bool, ClientError>> + Send {
        let mut client = self.client.clone();
        async move {
            let response = client
                .is_authenticated(Empty {})
                .await
                .map_err(|e| ClientError::ServerError(e.message().to_string()))?;

            Ok(response.into_inner().is_authenticated)
        }
    }

    fn start_initial_auth_flow(&self) -> impl Future<Output=Result<String, ClientError>> + Send {
        let mut client = self.client.clone();
        async move {
            let response = client
                .start_initial_auth_flow(Empty {})
                .await
                .map_err(|e| ClientError::ServerError(e.message().to_string()))?;

            Ok(response.into_inner().auth_url)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
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
}