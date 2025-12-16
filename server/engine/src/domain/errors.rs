use thiserror::Error;
use tokio::sync::oneshot;
use common::domain::errors::CommonError;
use common::domain::text_keys::TextKeys;

#[macro_export]
macro_rules! error {
    ($key:ident $(, $param:ident => $val:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::domain::errors::{ServerError, ErrorParam};
        use common::domain::text_keys::TextKeys;

        ServerError::Localized {
            key: TextKeys::$key,
            args: vec![
                $(
                    (ErrorParam::$param, $val.to_string()),
                )*
            ],
        }
    }};
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorParam {
    Url,
    Reason,
    Token,
}

impl ErrorParam {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorParam::Url => "url",
            ErrorParam::Reason => "reason",
            ErrorParam::Token => "token",
        }
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Localized error: {key}")]
    Localized {
        key: TextKeys,
        args: Vec<(ErrorParam, String)>,
    },

    #[error(transparent)]
    Common(#[from] CommonError),

    #[error("Invalid URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Callback channel closed unexpectedly")]
    CallbackRecv(#[from] oneshot::error::RecvError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),


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

    #[error("Missing redirect URL environment variable")]
    MissingRedirectUrlEnvVar,

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

    #[error("Could not find tokens in preferred store")]
    NoTokensFoundInStore,

    #[error("Invalid redirect URL: {0}")]
    InvalidRedirectUrl(String),

    #[error("Failed to start callback server: {0}")]
    ServerFailed(String),

    #[error("Server failed to signal ready state")]
    ServerFailedToSignalReadyState,

    #[error("Flow not started")]
    FlowNotStarted,

    #[error("Failed to exchange code for tokens: {0}")]
    TokenRequestFailed(String),

    #[error("No refresh token received")]
    NoRefreshTokenReceived,

    #[error("No access token received")]
    NoAccessTokenReceived,
}
