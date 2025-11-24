use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use crate::domain::configurator_defaults::{DEFAULT_CLIENT_ID, DEFAULT_CLIENT_SECRET};
use crate::domain::errors::ConfigurationError;
use crate::ports::driven::configurator_driven_port::ConfiguratorPort;

pub struct Configurator<CP: ConfiguratorPort> {
    configurator_port: CP
}

impl <CP: ConfiguratorPort>Configurator<CP> {
    pub fn new(configurator_port: CP) -> Self {
        Configurator { configurator_port }
    }

    fn auth_url(&self) -> &AuthUrl {
        &self.configurator_port.auth_url()
    }

    fn token_url(&self) -> &TokenUrl {
        &self.configurator_port.token_url()
    }

    fn client_id(&self) -> Result<&ClientId, ConfigurationError> {
        let client_id = self.configurator_port.client_id();
        if client_id.as_str() == DEFAULT_CLIENT_ID {
           Err(ConfigurationError::MissingClientIDEnvVarDuringBuild)
        }
        else {
            Ok(client_id)
        }
    }

    fn client_secret(&self) -> Result<&ClientSecret, ConfigurationError> {
        let client_secret = self.configurator_port.client_secret();
        if client_secret.secret().as_str() == DEFAULT_CLIENT_SECRET {
            Err(ConfigurationError::MissingClientSecretEnvVarDuringBuild)
        }
        else {
            Ok(client_secret)
        }
    }

    fn redirect_url(&self) -> &RedirectUrl {
        &self.configurator_port.redirect_url()
    }
}

#[cfg(test)]
mod tests {
    use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
    use crate::domain::configurator::Configurator;
    use crate::domain::configurator_defaults::{DEFAULT_AUTH_URL, DEFAULT_CLIENT_ID, DEFAULT_CLIENT_SECRET, DEFAULT_REDIRECT_URL, DEFAULT_TOKEN_URL};
    use crate::domain::errors::ConfigurationError;
    use crate::ports::driven::configurator_driven_port::ConfiguratorPort;

    struct FakeConfigurator {
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_url: RedirectUrl,
        auth_url: AuthUrl,
        token_url: TokenUrl
    }
    impl FakeConfigurator {
        pub fn new() -> Self {
            FakeConfigurator {
                auth_url: AuthUrl::new(DEFAULT_AUTH_URL.to_string()).unwrap(),
                token_url: TokenUrl::new(DEFAULT_TOKEN_URL.to_string()).unwrap(),
                client_id: ClientId::new(DEFAULT_CLIENT_ID.to_string()),
                client_secret: ClientSecret::new(DEFAULT_CLIENT_SECRET.to_string()),
                redirect_url: RedirectUrl::new(DEFAULT_REDIRECT_URL.to_string()).unwrap()
            }
        }
    }
    impl ConfiguratorPort for FakeConfigurator {

        fn auth_url(&self) -> &oauth2::AuthUrl {
            &self.auth_url
        }

        fn token_url(&self) -> &oauth2::TokenUrl {
            &self.token_url
        }

        fn client_id(&self) -> &oauth2::ClientId {
            &self.client_id
        }

        fn client_secret(&self) -> &ClientSecret {
            &self.client_secret
        }

        fn redirect_url(&self) -> &RedirectUrl {
            &self.redirect_url
        }
    }

    #[test]
    fn the_configurator_can_return_the_auth_url() {
        // Given a configurator adapter implemented using de default values
        let adapter = FakeConfigurator::new();

        // And a configurator is created that uses the adapter
        let configurator = Configurator::new(adapter);

        // When the auth url is requested
        let auth_url = configurator.auth_url();

        // Then the auth is wat was expected
        assert_eq!(auth_url.to_string(), DEFAULT_AUTH_URL);
    }

    #[test]
    fn the_configurator_can_return_the_token_url() {
        // Given a configurator adapter implemented using de default values
        let adapter = FakeConfigurator::new();

        // And a configurator is created that uses the adapter
        let configurator = Configurator::new(adapter);

        // When the auth url is requested
        let token_url = configurator.token_url();

        // Then the auth is wat was expected
        assert_eq!(token_url.to_string(), DEFAULT_TOKEN_URL);
    }

    #[test]
    fn when_not_setting_the_client_id_during_build_an_error_is_returned() {
        // Given a configurator adapter implemented using de default values
        let adapter = FakeConfigurator::new();

        // When a configurator is created that uses the adapter
        let configurator = Configurator::new(adapter);

        // Then a error is returned when getting the client id
        assert_eq!(configurator.client_id(), Err(ConfigurationError::MissingClientIDEnvVarDuringBuild));
    }

    #[test]
    fn when_not_setting_the_client_secret_during_build_an_error_is_returned() {
        // Given a configurator adapter implemented using de default values
        let adapter = FakeConfigurator::new();

        // When a configurator is created that uses the adapter
        let configurator = Configurator::new(adapter);

        // Then a error is returned when getting the client id
        assert!(configurator.client_secret().is_err());
    }

    #[test]
    fn the_configurator_can_return_the_redirect_url() {
        // Given a configurator adapter implemented using de default values
        let adapter = FakeConfigurator::new();

        // And a configurator is created that uses the adapter
        let configurator = Configurator::new(adapter);

        // When the auth url is requested
        let redirect_url = configurator.redirect_url();

        // Then the auth is wat was expected
        assert_eq!(redirect_url.to_string(), DEFAULT_REDIRECT_URL);
    }



}