use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};
use crate::domain::default_values::configurator_defaults::{DEFAULT_CLIENT_ID};
use crate::domain::errors::ServerError;
use crate::ports::driven::configurator_driven_port::ConfiguratorPort;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub client_id: ClientId,
    pub redirect_url: RedirectUrl,
}

#[derive(Debug, Clone)]
pub struct Configurator {
    config: Configuration,
}

impl Configurator {
    pub fn load<P: ConfiguratorPort>(port: &P) -> Result<Self, ServerError> {
        let config = port.load()?;

        if config.client_id.as_str() == DEFAULT_CLIENT_ID {
            return Err(ServerError::MissingClientIDEnvVarDuringBuild);
        }

        Ok(Configurator { config })
    }

    pub fn auth_url(&self) -> &AuthUrl { &self.config.auth_url }
    pub fn token_url(&self) -> &TokenUrl { &self.config.token_url }
    pub fn client_id(&self) -> &ClientId { &self.config.client_id }
    pub fn redirect_url(&self) -> &RedirectUrl { &self.config.redirect_url }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::default_values::configurator_defaults::*;
    use crate::domain::test_helpers::fake_configurator_adapter::FakeConfiguratorPort;

    #[test]
    fn the_configurator_can_return_the_auth_url() {
        let port = FakeConfiguratorPort::with_client_id("real-client-id");
        let configurator = Configurator::load(&port).unwrap();

        assert_eq!(configurator.auth_url().to_string(), DEFAULT_AUTH_URL);
    }

    #[test]
    fn the_configurator_can_return_the_token_url() {
        let port = FakeConfiguratorPort::with_client_id("real-client-id");
        let configurator = Configurator::load(&port).unwrap();

        assert_eq!(configurator.token_url().to_string(), DEFAULT_TOKEN_URL);
    }

    #[test]
    fn the_configurator_can_return_the_redirect_url() {
        let port = FakeConfiguratorPort::with_client_id("real-client-id");
        let configurator = Configurator::load(&port).unwrap();

        assert_eq!(configurator.redirect_url().as_str(), DEFAULT_REDIRECT_URL);
    }

    #[test]
    fn the_configurator_can_return_the_client_id() {
        let port = FakeConfiguratorPort::with_client_id("real-client-id");
        let configurator = Configurator::load(&port).unwrap();

        assert_eq!(configurator.client_id().as_str(), "real-client-id");
    }

    #[test]
    fn when_not_setting_the_client_id_during_build_an_error_is_returned() {
        let port = FakeConfiguratorPort::with_default_client_id();

        let result = Configurator::load(&port);

        assert!(matches!(
            result.unwrap_err(),
            ServerError::MissingClientIDEnvVarDuringBuild
        ));
    }
}