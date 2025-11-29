// engine/src/domain/test_helpers/fake_authenticator_adapter.rs
use async_trait::async_trait;
use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};
use crate::domain::errors::AuthFlowError;
use crate::domain::test_helpers::fake_configurator_adapter::FakeConfiguratorPort;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driven::configurator_driven_port::ConfiguratorPort;

#[allow(dead_code)]
pub struct FakeAuthenticatorDrivenAdapter {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
    redirect_url: RedirectUrl,
}

impl FakeAuthenticatorDrivenAdapter {
    pub fn new_default() -> Self {
        let port = FakeConfiguratorPort::with_client_id("test-client-id");
        let config = port.load().unwrap();

        FakeAuthenticatorDrivenAdapter::new(
            config.auth_url,
            config.token_url,
            config.client_id,
            config.redirect_url,
        )
    }
}

#[async_trait]
impl AuthenticatorDrivenPort for FakeAuthenticatorDrivenAdapter {
    fn new(auth_url: AuthUrl, token_url: TokenUrl, client_id: ClientId, redirect_url: RedirectUrl) -> Self {
        FakeAuthenticatorDrivenAdapter { auth_url, token_url, client_id, redirect_url }
    }

    async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        todo!()
    }

    async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        todo!()
    }
}