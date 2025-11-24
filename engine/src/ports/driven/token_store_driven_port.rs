use crate::domain::errors::ConfigurationError;

pub trait TokenStoreDrivenPort {
    fn load() -> Result<Self, ConfigurationError>
    where
        Self: Sized;

    fn save(&self) -> Result<(), ConfigurationError>;

    fn access_token(&self) -> String;
    fn refresh_token(&self) -> String;
    fn expires_at(&self) -> i64;
}