use i18n_embed::I18nEmbedError;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::domain::text_keys::TextKeys;
use crate::ports::i18n_driven_port::I18nDrivenPort;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
pub struct ApplicationError {
    pub text_key: TextKeys,
    pub detail: Option<String>,
}

#[macro_export]
macro_rules! application_error {
    // Key only
    ($text_key:expr) => {
        $crate::domain::errors::ApplicationError {
            text_key: $text_key,
            detail: None,
        }
    };

    ($text_key:expr, $detail:expr) => {
        $crate::domain::errors::ApplicationError {
            text_key: $text_key,
            detail: Some($detail.to_string()),
        }
    };

    // Key & detail
    ($text_key:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::domain::errors::ApplicationError {
            text_key: $text_key,
            detail: Some(format!($fmt, $($arg)*)),
        }
    };
}

pub fn translate_error<I18nPort: I18nDrivenPort>(err: &ApplicationError, i18n: &I18nPort) -> String {
    let base_message = i18n.t(err.text_key);

    match &err.detail {
        Some(detail) if !detail.is_empty() => {
            format!("{base_message}: {detail}")
        }
        _ => base_message
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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