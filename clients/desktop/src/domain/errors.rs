use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
use common::domain::errors::LocalizedError;
use common::ports::i18n_driven_port::I18nDrivenPort;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! error {
    ($key:ident $(, $param:ident => $val:expr )* $(,)?) => {{
        let localized = common::localized_error!($key $(, $param => $val)*);
        $crate::domain::errors::ClientError::Localized(localized)
    }};
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClientError {
    Localized(LocalizedError),
    ServerError(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Localized(err) => write!(f, "Localized error: {:?}", err.key),
            ClientError::ServerError(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn translate_error<I18nPort: I18nDrivenPort>(err: &ClientError, i18n: &I18nPort) -> String {
    match err {
        ClientError::ServerError(msg) => msg.clone(),
        ClientError::Localized(loc) => {
            let flat_args: Vec<(&'static str, String)> = loc
                .args
                .iter()
                .map(|(param, value)| (param.as_str(), value.clone()))
                .collect();
            i18n.t_with_args(loc.key, &flat_args)
        }
    }
}