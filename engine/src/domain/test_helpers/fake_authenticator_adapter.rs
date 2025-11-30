// engine/src/domain/test_helpers/fake_authenticator_adapter.rs
use async_trait::async_trait;
use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};
use url::Url;
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
    pub fn new(auth_url: AuthUrl, token_url: TokenUrl, client_id: ClientId, redirect_url: RedirectUrl) -> Self {
        FakeAuthenticatorDrivenAdapter { auth_url, token_url, client_id, redirect_url }
    }
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
    async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        let mut url = Url::parse(self.auth_url.as_str())
            .map_err(|e| AuthFlowError::InvalidRedirectUrl(e.to_string()))?;
        url.query_pairs_mut()
            .append_pair("client_id", self.client_id.as_str());
        Ok(url.to_string())
    }

    async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        Ok(true)
    }
}