use async_trait::async_trait;
use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};
use crate::domain::errors::AuthFlowError;
use crate::domain::test_helpers::fake_configurator_adapter::FakeConfiguratorDrivenAdapter;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driven::configurator_driven_port::ConfiguratorPort;

pub struct FakeAuthenticatorDrivenAdapter {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
    redirect_url: RedirectUrl,
}

impl FakeAuthenticatorDrivenAdapter {
    pub fn new_default() -> Self {
        let configurator = FakeConfiguratorDrivenAdapter::new();
        
        FakeAuthenticatorDrivenAdapter::new(
            configurator.auth_url().clone(),
            configurator.token_url().clone(),
            configurator.client_id().clone(),
            configurator.redirect_url().clone()
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