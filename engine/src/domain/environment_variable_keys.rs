use crate::domain::errors::ConfigurationError;

pub enum EnvironmentVariableKeys {
    // ApiUrl,
    ClientID,
    ClientSecret,
    AuthUrl,
    TokenUrl,
    RedirectUrl,
}

impl EnvironmentVariableKeys {
    pub fn key(&self) -> &str {
        match *self {
            // Self::ApiUrl => "API_URL",
            Self::AuthUrl => "AUTH_URL",
            Self::TokenUrl => "TOKEN_URL",
            Self::ClientID => "CLIENT_ID",
            Self::ClientSecret => "CLIENT_SECRET",
            Self::RedirectUrl => "REDIRECT_URL",
        }
    }

    pub fn error(&self) -> ConfigurationError {
        match *self {
            // Self::ApiUrl => ConfigurationError::MissingApiUrlEnvVar,
            Self::AuthUrl => ConfigurationError::MissingAuthUrlEnvVar,
            Self::TokenUrl => ConfigurationError::MissingTokenUrlEnvVar,
            Self::ClientID => ConfigurationError::MissingClientIDEnvVar,
            Self::ClientSecret => ConfigurationError::MissingClientSecretEnvVar,
            Self::RedirectUrl => ConfigurationError::MissingRedirectUrlEnvVar,
        }
    }
}