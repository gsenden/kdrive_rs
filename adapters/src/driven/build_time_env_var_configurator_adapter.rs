use engine::domain::errors::ConfigurationError;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use engine::ports::driven::configurator_driven_port::{ConfiguratorFactoryPort, ConfiguratorPort};
use engine::domain::configurator_defaults::*;

#[derive(Debug)]
pub struct BuildTimeEnvVarConfigurator {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_url: RedirectUrl,
}

impl ConfiguratorPort for BuildTimeEnvVarConfigurator {
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

pub struct BuildTimeEnvVarConfiguratorFactory;
impl ConfiguratorFactoryPort for BuildTimeEnvVarConfiguratorFactory {
    type Port = BuildTimeEnvVarConfigurator;

    fn load(&self) -> Result<Self::Port, ConfigurationError>
    where
        Self: Sized
    {
        let auth_url = option_env!("AUTH_URL").unwrap_or(DEFAULT_AUTH_URL).to_string();
        let token_url = option_env!("TOKEN_URL").unwrap_or(DEFAULT_TOKEN_URL).to_string();
        let client_id = option_env!("CLIENT_ID").unwrap_or(DEFAULT_CLIENT_ID).to_string();
        let client_secret = option_env!("CLIENT_SECRET").unwrap_or(DEFAULT_CLIENT_SECRET).to_string();
        let redirect_url = option_env!("REDIRECT_URL").unwrap_or(DEFAULT_REDIRECT_URL).to_string();

        Ok(BuildTimeEnvVarConfigurator {
            auth_url: AuthUrl::new(auth_url)?,
            token_url: TokenUrl::new(token_url)?,
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            redirect_url: RedirectUrl::new(redirect_url)?
        })
    }
}

#[cfg(test)]
mod tests {
    use engine::ports::driven::configurator_driven_port::{ConfiguratorFactoryPort, ConfiguratorPort};
    use crate::driven::build_time_env_var_configurator_adapter::*;
    use engine::domain::configurator_defaults::*;

    #[test]
    fn client_id_from_env() {
        // When the environment variables are loaded
        let factory = BuildTimeEnvVarConfiguratorFactory;
        let environment_variables = factory.load().unwrap();

        // Then the client id is loaded from the env file
        assert_eq!(environment_variables.client_id().to_string(), DEFAULT_CLIENT_ID);
    }

    #[test]
    fn client_secret_from_env() {
        // When the environment variables are loaded
        let factory = BuildTimeEnvVarConfiguratorFactory;
        let environment_variables = factory.load().unwrap();

        // Then the client secret is loaded from the env file
        assert_eq!(environment_variables.client_secret().secret(), DEFAULT_CLIENT_SECRET);
    }

    #[test]
    fn token_url_from_env() {
        // When the environment variables are loaded
        let factory = BuildTimeEnvVarConfiguratorFactory;
        let environment_variables = factory.load().unwrap();

        // Then the token url loaded from the env file
        assert_eq!(environment_variables.token_url().to_string(), DEFAULT_TOKEN_URL);
    }

    #[test]
    fn redirect_url_from_env() {
        // When the environment variables are loaded
        let factory = BuildTimeEnvVarConfiguratorFactory;
        let environment_variables = factory.load().unwrap();

        // Then the token url loaded from the env file
        assert_eq!(environment_variables.redirect_url().as_str(), DEFAULT_REDIRECT_URL);
    }

}
