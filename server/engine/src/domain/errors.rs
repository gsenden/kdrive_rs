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

}
