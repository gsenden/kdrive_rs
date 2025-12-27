use async_trait::async_trait;
use common::domain::errors::ApplicationError;

#[async_trait]
pub trait UIDrivingPort: Send + Sync {
    async fn on_login_view_shown(&self) -> Result<String, ApplicationError>;
}