// engine/src/domain/test_helpers/fake_configurator_adapter.rs
use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};
use crate::domain::configuration::Configuration;
use crate::domain::default_values::configurator_defaults::*;
use crate::domain::errors::ConfigurationError;
use crate::ports::driven::configurator_driven_port::ConfiguratorPort;

pub struct FakeConfiguratorPort {
    client_id: String,
}

impl FakeConfiguratorPort {
    pub fn with_client_id(client_id: &str) -> Self {
        FakeConfiguratorPort {
            client_id: client_id.to_string(),
        }
    }

    pub fn with_default_client_id() -> Self {
        FakeConfiguratorPort {
            client_id: DEFAULT_CLIENT_ID.to_string(),
        }
    }
}

impl ConfiguratorPort for FakeConfiguratorPort {
    fn load(&self) -> Result<Configuration, ConfigurationError> {
        Ok(Configuration {
            auth_url: AuthUrl::new(DEFAULT_AUTH_URL.to_string())?,
            token_url: TokenUrl::new(DEFAULT_TOKEN_URL.to_string())?,
            client_id: ClientId::new(self.client_id.clone()),
            redirect_url: RedirectUrl::new(DEFAULT_REDIRECT_URL.to_string())?,
        })
    }
}