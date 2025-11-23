use crate::domain::errors::ConfigurationError;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub trait EnvironmentVariablesPort {
    fn auth_url(&self) -> &AuthUrl;
    fn token_url(&self) -> &TokenUrl;
    fn client_id(&self) -> &ClientId;
    fn client_secret(&self) -> &ClientSecret;
    fn redirect_url(&self) -> &RedirectUrl;
}

pub trait EnvironmentVariablesFactoryPort {
    type Port: EnvironmentVariablesPort;
    fn load(&self, env_file_option: Option<&str>) -> Result<Self::Port, ConfigurationError>
    where
        Self: Sized;
}