use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use crate::domain::default_values::configurator_defaults::*;
use crate::ports::driven::configurator_driven_port::ConfiguratorPort;

pub struct FakeConfiguratorDrivenAdapter {
    client_id: ClientId,
    redirect_url: RedirectUrl,
    auth_url: AuthUrl,
    token_url: TokenUrl
}
impl FakeConfiguratorDrivenAdapter {
    pub fn new() -> Self {
        FakeConfiguratorDrivenAdapter {
            auth_url: AuthUrl::new(DEFAULT_AUTH_URL.to_string()).unwrap(),
            token_url: TokenUrl::new(DEFAULT_TOKEN_URL.to_string()).unwrap(),
            client_id: ClientId::new(DEFAULT_CLIENT_ID.to_string()),
            redirect_url: RedirectUrl::new(DEFAULT_REDIRECT_URL.to_string()).unwrap()
        }
    }
}
impl ConfiguratorPort for FakeConfiguratorDrivenAdapter {

    fn auth_url(&self) -> &oauth2::AuthUrl {
        &self.auth_url
    }

    fn token_url(&self) -> &oauth2::TokenUrl {
        &self.token_url
    }

    fn client_id(&self) -> &oauth2::ClientId {
        &self.client_id
    }

    fn redirect_url(&self) -> &RedirectUrl {
        &self.redirect_url
    }
}

