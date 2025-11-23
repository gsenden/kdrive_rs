use crate::domain::errors::ConfigurationError;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub trait EnvironmentVariablesPort {
    fn auth_url(&self) -> &AuthUrl;
    fn token_url(&self) -> &TokenUrl;
    fn client_id(&self) -> &ClientId;
    fn client_secret(&self) -> &ClientSecret;
    fn redirect_url(&self) -> &RedirectUrl;
    fn defaults_overwritten_by_env(&self) -> bool;
}

pub trait EnvironmentVariablesFactoryPort {
    type Port: EnvironmentVariablesPort;

    fn load(&self) -> Result<Self::Port, ConfigurationError>
    where
        Self: Sized;
}