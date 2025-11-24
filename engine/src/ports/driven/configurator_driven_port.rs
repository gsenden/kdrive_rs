use crate::domain::errors::ConfigurationError;
use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};

pub trait ConfiguratorPort {
    fn auth_url(&self) -> &AuthUrl;
    fn token_url(&self) -> &TokenUrl;
    fn client_id(&self) -> &ClientId;
    fn redirect_url(&self) -> &RedirectUrl;
}

pub trait ConfiguratorFactoryPort {
    type Port: ConfiguratorPort;

    fn load(&self) -> Result<Self::Port, ConfigurationError>
    where
        Self: Sized;
}