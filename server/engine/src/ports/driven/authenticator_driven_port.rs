use async_trait::async_trait;
use common::domain::errors::ApplicationError;
use crate::domain::tokens::Tokens;

#[async_trait]
pub trait AuthenticatorDrivenPort {
    async fn start_initial_auth_flow(&mut self) -> Result<String, ApplicationError>;
    async fn continue_initial_auth_flow(&mut self) -> Result<bool, ApplicationError>;
    async fn get_tokens(&self) -> Result<Tokens, ApplicationError>;
}