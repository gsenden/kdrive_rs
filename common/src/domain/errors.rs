use i18n_embed::I18nEmbedError;
use thiserror::Error;

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
