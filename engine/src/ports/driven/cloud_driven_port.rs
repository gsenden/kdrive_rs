use crate::domain::auth::AuthUrl;
use crate::domain::errors::{AuthFlowError, ConfigurationError};
use crate::ports::driven::environment_variables_port::EnvironmentVariablesPort;
use async_trait::async_trait;

#[async_trait]
pub trait CloudDrivenPort : Sized {
    fn new<E: EnvironmentVariablesPort>(environment_variables: &E) -> Result<Self, ConfigurationError>;
    fn list_files(&self) -> Vec<String>;
    fn get_authentication_url_to_be_opened_by_user(&self) -> AuthUrl;
    async fn get_authorization_code(&self) -> Result<String, AuthFlowError>;
}

