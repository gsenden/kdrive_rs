use async_trait::async_trait;
use common::domain::errors::ApplicationError;
use crate::ports::driving::ui_driving_port::UIDrivingPort;
use crate::domain::client::Client;
use crate::ports::driven::server_driven_port::ServerDrivenPort;

pub struct UIAdapter<ServerPort>
where
    ServerPort: ServerDrivenPort,
{
    client: Client<ServerPort>,
}

impl<ServerPort> UIAdapter<ServerPort>
where
    ServerPort: ServerDrivenPort,
{
    pub fn new(client: Client<ServerPort>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<ServerPort> UIDrivingPort for UIAdapter<ServerPort>
where
    ServerPort: ServerDrivenPort + Send + Sync,
{
    async fn on_login_view_shown(&self) -> Result<String, ApplicationError> {
        self.client.on_login_view_shown().await
    }
}