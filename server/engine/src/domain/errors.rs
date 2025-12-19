use thiserror::Error;
use tokio::sync::oneshot;
use common::domain::errors::{CommonError, LocalizedError};

#[macro_export]
macro_rules! error {
    ($key:ident $(, $param:ident => $val:expr )* $(,)?) => {{
        let localized = common::localized_error!($key $(, $param => $val)*);
        $crate::domain::errors::ServerError::Localized(localized)
    }};
}

#[derive(Debug, Error)]
pub enum ServerError {
    // Veranderd van { key, args } naar (LocalizedError)
    #[error("Localized error: {0:?}")]
    Localized(LocalizedError),

    #[error(transparent)]
    Common(#[from] CommonError),

    #[error("Invalid URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Callback channel closed unexpectedly")]
    CallbackRecv(#[from] oneshot::error::RecvError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
