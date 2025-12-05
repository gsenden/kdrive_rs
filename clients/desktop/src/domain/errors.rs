use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ClientError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
}