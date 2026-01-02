use async_trait::async_trait;
use common::domain::errors::ApplicationError;
use crate::domain::test_helpers::fake_token_store_adapter::{TEST_RING_ACCESS_TOKEN, TEST_RING_EXPIRES_AT, TEST_RING_REFRESH_TOKEN};
use crate::domain::tokens::Tokens;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;

pub struct SlowAuthenticatorAdapter;

#[async_trait]
impl AuthenticatorDrivenPort for SlowAuthenticatorAdapter {
    async fn start_initial_auth_flow(&mut self) -> Result<String, ApplicationError> {
        Ok("http://example.com".to_string())
    }

    async fn continue_initial_auth_flow(&mut self) -> Result<(), ApplicationError> {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        Ok(())
    }

    async fn get_tokens(&self) -> Result<Tokens, ApplicationError> {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        Ok(Tokens {
            access_token: TEST_RING_ACCESS_TOKEN.parse().unwrap(),
            refresh_token: TEST_RING_REFRESH_TOKEN.parse().unwrap(),
            expires_at: TEST_RING_EXPIRES_AT,
        })
    }
}