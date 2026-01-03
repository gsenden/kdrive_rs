use common::domain::errors::ApplicationError;

#[async_trait::async_trait]
pub trait AuthenticatorDrivingPort {
    fn is_authenticated(&self) -> bool;
    async fn start_initial_auth_flow(&mut self) -> Result<String, ApplicationError>;
    async fn continue_initial_auth_flow(&mut self);
}