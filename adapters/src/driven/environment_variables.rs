use engine::domain::errors::ConfigurationError;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use engine::domain::environment_variable_keys::EnvironmentVariableKeys;
use engine::ports::driven::environment_variables_port::{EnvironmentVariablesFactoryPort, EnvironmentVariablesPort};

#[derive(Debug)]
pub struct EnvironmentVariables {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_url: RedirectUrl,
}

impl EnvironmentVariablesPort for EnvironmentVariables {
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

pub struct EnvironmentVariablesFactory;
impl EnvironmentVariablesFactoryPort for EnvironmentVariablesFactory {
    type Port = EnvironmentVariables;

    fn load(&self, env_file_option: Option<&str>) -> Result<Self::Port, ConfigurationError>
    where
        Self: Sized
    {
        if let Some(env_file) = env_file_option {
            dotenvy::from_filename(env_file)
                .map_err(|_| ConfigurationError::MissingEnvFile)?;
        } else {
            dotenvy::dotenv().ok();
        }

        let read_environment_variable = |env_name: &EnvironmentVariableKeys| {
            dotenvy::var(env_name.key()).map_err(|_| env_name.error())
        };

        Ok(EnvironmentVariables {
            // api_url: read_environment_variable(&EnvironmentVariableKeys::ApiUrl)?,
            auth_url: AuthUrl::new(read_environment_variable(&EnvironmentVariableKeys::AuthUrl)?)?,
            token_url: TokenUrl::new(read_environment_variable(&EnvironmentVariableKeys::TokenUrl)?)?,
            client_id: ClientId::new(read_environment_variable(&EnvironmentVariableKeys::ClientID)?),
            client_secret: ClientSecret::new(read_environment_variable(&EnvironmentVariableKeys::ClientSecret)?),
            redirect_url: RedirectUrl::new(read_environment_variable(&EnvironmentVariableKeys::RedirectUrl)?)?
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use engine::domain::errors::ConfigurationError;
//     use engine::ports::driven::environment_variables_port::{EnvironmentVariablesFactoryPort, EnvironmentVariablesPort};
//     use crate::driven::environment_variables::{EnvironmentVariablesFactory};
//     use crate::test_assets::constants::*;
// 
//     pub const VALID_TEST_ENV_FILE: &str = "test_assets/.test.env";
// 
//     #[test]
//     fn kdrive_returns_an_error_when_env_file_cannot_be_found() {
//         // Given a non-existing env file
//         let non_existing_env_file = "test_assets/.env.non-existing";
// 
//         // When the environment variables are loaded
//         let factory = EnvironmentVariablesFactory;
//         let result = factory.load(Some(non_existing_env_file));
// 
// 
//         // Then an error is returned that the env file cannot be found
//         assert_eq!(result.unwrap_err(), ConfigurationError::MissingEnvFile);
//     }
// 
// 
//     #[test]
//     fn kdrive_takes_the_client_id_from_env() {
//         // When the environment variables are loaded
//         let factory = EnvironmentVariablesFactory;
//         let environment_variables = factory.load(Some(VALID_TEST_ENV_FILE))
//             .unwrap();
// 
//         // Then the client id is loaded from the env file
//         assert_eq!(environment_variables.client_id().to_string(), VALID_TEST_CLIENT_ID);
//     }
// 
//     #[test]
//     fn kdrive_takes_the_client_secret_from_env() {
//         // When the environment variables are loaded
//         let factory = EnvironmentVariablesFactory;
//         let environment_variables = factory.load(Some(VALID_TEST_ENV_FILE))
//             .unwrap();
// 
//         // Then the client secret is loaded from the env file
//         assert_eq!(environment_variables.client_secret().secret(), VALID_TEST_CLIENT_SECRET);
//     }
// 
//     #[test]
//     fn kdrive_takes_the_token_url_from_env() {
//         // When the environment variables are loaded
//         let factory = EnvironmentVariablesFactory;
//         let environment_variables = factory.load(Some(VALID_TEST_ENV_FILE))
//             .unwrap();
// 
//         // Then the token url loaded from the env file
//         assert_eq!(environment_variables.token_url().to_string(), VALID_TEST_TOKEN_URL);
//     }
// 
//     #[test]
//     fn kdrive_takes_the_redirect_url_from_env() {
//         // When the environment variables are loaded
//         let factory = EnvironmentVariablesFactory;
//         let environment_variables = factory.load(Some(VALID_TEST_ENV_FILE))
//             .unwrap();
// 
//         // Then the token url loaded from the env file
//         assert_eq!(environment_variables.redirect_url().as_str(), VALID_TEST_REDIRECT_URL);
//     }
// 
//     #[test]
//     fn kdrive_can_start_the_auth_flow() {
//         // When the environment variables are loaded
//         let factory = EnvironmentVariablesFactory;
//         let environment_variables = factory.load(Some(VALID_TEST_ENV_FILE))
//             .unwrap();
// 
//         // Then a valid auth flow is returned
//         assert!(environment_variables.auth_url().as_str().starts_with(VALID_TEST_AUTH_URL));
//     }
// }
