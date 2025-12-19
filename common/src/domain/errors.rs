use i18n_embed::I18nEmbedError;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::domain::text_keys::TextKeys;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Failed to initialize I18N. In language: {0}, missing key: {1}")]
    I18NMissingKeyError(String, String),
    #[error("Failed to initialize I18N. Language: {0}, with key: {1} has no value")]
    I18NMissingValueError(String, String),
    #[error("Failed to initialize I18N. Language: {0} is missing")]
    I18NMissingLanguage(String),
    #[error("I18N initialization failed: {0}")]
    I18n(#[from] I18nEmbedError),
}

#[macro_export]
macro_rules! localized_error {
    ($key:ident $(, $param:ident => $val:expr )* $(,)?) => {{
        use $crate::domain::errors::{LocalizedError, ErrorParam};
        use $crate::domain::text_keys::TextKeys;

        LocalizedError {
            key: TextKeys::$key,
            args: vec![
                $(
                    (ErrorParam::$param, $val.to_string()),
                )*
            ],
        }
    }};
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalizedError {
    pub key: TextKeys,
    pub args: Vec<(ErrorParam, String)>,
}