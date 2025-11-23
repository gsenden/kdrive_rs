use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use crate::domain::errors::AuthFlowError;
use async_trait::async_trait;

#[async_trait]
pub trait AuthenticatorDrivenPort {
    fn new(auth_url: AuthUrl, token_url: TokenUrl, client_id: ClientId, client_secret: ClientSecret, redirect_url: RedirectUrl) -> Self;
    async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError>;
    async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError>;
}