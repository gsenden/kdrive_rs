use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Configuration error: {0}")]
    Configuration(#[from] engine::domain::errors::ConfigurationError),

    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error("Invalid socket address: {0}")]
    InvalidAddress(#[from] std::net::AddrParseError),
}