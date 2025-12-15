// adapters/src/driven/build_time_env_var_configurator_adapter.rs
use engine::domain::configuration::Configuration;
use engine::domain::errors::ServerError;
use engine::domain::default_values::configurator_defaults::*;
use engine::ports::driven::configurator_driven_port::ConfiguratorPort;
use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};

pub struct BuildTimeEnvVarConfiguratorPort;

impl ConfiguratorPort for BuildTimeEnvVarConfiguratorPort {
    fn load(&self) -> Result<Configuration, ServerError> {
        let auth_url = option_env!("AUTH_URL")
            .unwrap_or(DEFAULT_AUTH_URL)
            .to_string();
        let token_url = option_env!("TOKEN_URL")
            .unwrap_or(DEFAULT_TOKEN_URL)
            .to_string();
        let client_id = option_env!("CLIENT_ID")
            .unwrap_or(DEFAULT_CLIENT_ID)
            .to_string();
        let redirect_url = option_env!("REDIRECT_URL")
            .unwrap_or(DEFAULT_REDIRECT_URL)
            .to_string();

        Ok(Configuration {
            auth_url: AuthUrl::new(auth_url)?,
            token_url: TokenUrl::new(token_url)?,
            client_id: ClientId::new(client_id),
            redirect_url: RedirectUrl::new(redirect_url)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_url_from_env() {
        let port = BuildTimeEnvVarConfiguratorPort;
        let config = port.load().unwrap();

        assert_eq!(config.auth_url.to_string(), DEFAULT_AUTH_URL);
    }

    #[test]
    fn client_id_from_env() {
        let port = BuildTimeEnvVarConfiguratorPort;
        let config = port.load().unwrap();

        assert_eq!(config.client_id.as_str(), DEFAULT_CLIENT_ID);
    }

    #[test]
    fn token_url_from_env() {
        let port = BuildTimeEnvVarConfiguratorPort;
        let config = port.load().unwrap();

        assert_eq!(config.token_url.to_string(), DEFAULT_TOKEN_URL);
    }

    #[test]
    fn redirect_url_from_env() {
        let port = BuildTimeEnvVarConfiguratorPort;
        let config = port.load().unwrap();

        assert_eq!(config.redirect_url.as_str(), DEFAULT_REDIRECT_URL);
    }
}