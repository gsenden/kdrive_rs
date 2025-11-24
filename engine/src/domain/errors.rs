use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Configuration(#[from] ConfigurationError),

    #[error("Auth error: {0}")]
    Auth(#[from] AuthFlowError),
}

#[derive(Debug, Error, PartialEq)]
pub enum ConfigurationError {
    #[error("Missing API URL environment variable")]
    MissingApiUrlEnvVar,

    #[error("Missing .env file")]
    MissingEnvFile,

    #[error("Missing client ID environment variable")]
    MissingClientIDEnvVar,

    #[error("Missing client secret environment variable")]
    MissingClientSecretEnvVar,

    #[error("Missing auth URL environment variable")]
    MissingAuthUrlEnvVar,

    #[error("Missing token URL environment variable")]
    MissingTokenUrlEnvVar,

    #[error("Invalid URL: {0}")]
    ParseError(String),

    #[error("Missing redirect URL environment variable")]
    MissingRedirectUrlEnvVar,
    
    #[error("Environment variable CLIENT_ID needs to be set during build time")]
    MissingClientIDEnvVarDuringBuild,

    #[error("Environment variable CLIENT_SECRET needs to be set during build time")]
    MissingClientSecretEnvVarDuringBuild,

    #[error("At least one store port needs to be set")]
    MissingStorePort,

    #[error("Could not find config folder")]
    NoConfigFolderFound,

    #[error("Could not create folder: {0}")]
    CouldNotCreateFolder(String),

    #[error("Could not read tokens from file: {0}")]
    CouldNotReadTokensFromFile(String),

    #[error("Could not parse json: {0}")]
    CouldNotParseJson(String),

    #[error("Could not serialize tokens: {0}")]
    CouldNotSerializeTokens(String),

    #[error("Could not open token file: {0}")]
    CouldNotOpenTokenFile(String),

    #[error("Could not save token file: {0}")]
    CouldNotSaveTokenFile(String),

    #[error("Could not read tokens from keyring: {0}")]
    CouldNotReadTokensFromKeyring(String),

    #[error("Could not save tokens to keyring: {0}")]
    CouldNotSaveTokensToKeyring(String),

    #[error("Could not access keyring: {0}")]
    CouldNotAccessKeyring(String),

}

impl From<url::ParseError> for ConfigurationError {
    fn from(err: url::ParseError) -> Self {
        ConfigurationError::ParseError(err.to_string())
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum AuthFlowError {
    #[error("Invalid redirect URL: {0}")]
    InvalidRedirectUrl(String),
    #[error("Missing redirect URL in the client")]
    MissingRedirectUrl,
    #[error("Failed to start callback server: {0}")]
    ServerFailed(String),
    #[error("Callback channel closed unexpectedly")]
    CallbackClosedUnexpectedly,
    #[error("OAuth returned error: {0}")]
    OAuthReturnedError(String),
    #[error("Missing authorization code")]
    MissingAuthorizationCode,
    #[error("Server failed to signal ready state")]
    ServerFailedToSignalReadyState,
    #[error("Flow not started")]
    FlowNotStarted,
    #[error("Failed to exchange code for tokens: {0}")]
    TokenRequestFailed(String),
}

impl From<oneshot::error::RecvError> for AuthFlowError {
    fn from(_: oneshot::error::RecvError) -> Self {
        AuthFlowError::ServerFailedToSignalReadyState
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum AuthError {
    #[error("Invalid URL: {0}")]
    UrlParseError(String),
}

impl From<url::ParseError> for AuthError {
    fn from(err: url::ParseError) -> Self {
        AuthError::UrlParseError(err.to_string())
    }
}