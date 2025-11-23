use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use engine::ports::driven::environment_variables_port::EnvironmentVariablesPort;
use crate::test_assets::constants::*;

pub struct FakeEnvironmentVariables {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_url: RedirectUrl,
}

impl Default for FakeEnvironmentVariables {
    fn default() -> Self {
        Self {
            auth_url: AuthUrl::new(VALID_TEST_AUTH_URL.to_string()).unwrap(),
            token_url: TokenUrl::new(VALID_TEST_TOKEN_URL.to_string()).unwrap(),
            client_id: ClientId::new(VALID_TEST_CLIENT_ID.to_string()),
            client_secret: ClientSecret::new(VALID_TEST_CLIENT_SECRET.to_string()),
            redirect_url: RedirectUrl::new(VALID_TEST_REDIRECT_URL.to_string()).unwrap(),
        }
    }
}

impl EnvironmentVariablesPort for FakeEnvironmentVariables {
    fn auth_url(&self) -> &AuthUrl {
        &self.auth_url
    }

    fn token_url(&self) -> &TokenUrl {
        &self.token_url
    }

    fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    fn redirect_url(&self) -> &RedirectUrl {
        &self.redirect_url
    }
}