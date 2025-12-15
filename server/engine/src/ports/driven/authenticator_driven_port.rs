use crate::domain::errors::ServerError;
use async_trait::async_trait;
use crate::domain::tokens::Tokens;

#[async_trait]
pub trait AuthenticatorDrivenPort {
    async fn start_initial_auth_flow(&mut self) -> Result<String, ServerError>;
    async fn continue_initial_auth_flow(&mut self) -> Result<bool, ServerError>;
    async fn get_tokens(&self) -> Result<Tokens, ServerError>;
}