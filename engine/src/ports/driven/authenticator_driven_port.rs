use crate::domain::errors::AuthFlowError;
use async_trait::async_trait;

#[async_trait]
pub trait AuthenticatorDrivenPort {
    async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError>;
    async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError>;
}