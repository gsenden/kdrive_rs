use std::future::Future;
use common::domain::defaults::DEFAULT_SERVER_URL;
use tonic::transport::Channel;

use crate::domain::errors::ClientError;
use crate::kdrive::kdrive_service_client::KdriveServiceClient;
use crate::kdrive::Empty;
use crate::ports::driven::server_driven_port::ServerDrivenPort;

#[derive(Clone)]
pub struct GrpcServerAdapter {
    client: KdriveServiceClient<Channel>,
}

impl PartialEq for GrpcServerAdapter {
    fn eq(&self, _other: &Self) -> bool {
        // We kunnen de clients niet echt vergelijken,
        // dus we zeggen dat ze altijd gelijk zijn
        // (of altijd ongelijk, afhankelijk van je use case)
        true
    }
}

impl GrpcServerAdapter {
    pub async fn connect() -> Result<Self, ClientError> {
        let channel = Channel::from_static(DEFAULT_SERVER_URL)
            .connect()
            .await
            .map_err(|e| ClientError::ConnectionFailed(e.to_string()))?;

        Ok(Self {
            client: KdriveServiceClient::new(channel),
        })
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