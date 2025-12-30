use std::collections::HashMap;
use std::string::ParseError;
use serde::{Deserialize, Serialize};
use i18n_loader::TextKeys::{ParserError, TransportError};
use crate::domain::text_keys::TextKeys;
use tonic::Status;
use tokio::sync::oneshot::error::RecvError;
use tonic::metadata::MetadataValue;
use crate::domain::defaults::APPLICATION_ERROR_DETAIL_FIELD_NAME;

use crate::kdrive::{
    ServerEvent,
    ApplicationErrorEvent,
};
use crate::kdrive::server_event::Event as ServerEventKind;
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

impl ApplicationError {
    pub fn translate<I18nPort>(&self, i18n: &I18nPort) -> String
    where
        I18nPort: I18nDrivenPort,
    {
        let base_message = i18n.t(self.text_key);

        match &self.detail {
            Some(detail) if !detail.is_empty() => {
                format!("{base_message}: {detail}")
            }
            _ => base_message,
        }
    }
}

impl From<ParseError> for ApplicationError {
    fn from(err: ParseError) -> Self {
        ApplicationError {
            text_key: ParserError,
            detail: Some(err.to_string()),
        }
    }
}

impl From<oauth2::url::ParseError> for ApplicationError {
    fn from(err: oauth2::url::ParseError) -> Self {
        ApplicationError {
            text_key: ParserError,
            detail: Some(err.to_string()),
        }
    }
}

impl From<RecvError> for ApplicationError {
    fn from(err: RecvError) -> Self {
        ApplicationError {
            text_key: ParserError,
            detail: Some(err.to_string()),
        }
    }
}

impl From<tonic::transport::Error> for ApplicationError {
    fn from(err: tonic::transport::Error) -> Self {
        ApplicationError {
            text_key: TransportError,
            detail: Some(err.to_string()),
        }
    }
}
impl From<ApplicationError> for Status {
    fn from(err: ApplicationError) -> Self {
        let message = err.text_key.to_string();
        let mut status = Status::invalid_argument(message);

        if let Some(detail) = err.detail {
            if let Ok(value) = detail.parse::<MetadataValue<_>>() {
                status
                    .metadata_mut()
                    .insert(APPLICATION_ERROR_DETAIL_FIELD_NAME, value);
            }
            // else: This should not happen and depends on From<Status>. There are
            //       tests for insurance.
        }

        status
    }
}

impl From<Status> for ApplicationError {
    fn from(status: Status) -> Self {
        let text_key = status
            .message()
            .parse::<TextKeys>()
            .unwrap_or(TextKeys::ConnectionErrorMessage);

        let detail = status
            .metadata()
            .get(APPLICATION_ERROR_DETAIL_FIELD_NAME)
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        ApplicationError {
            text_key,
            detail,
        }
    }
}

impl From<ApplicationError> for ServerEvent {
    fn from(err: ApplicationError) -> Self {
        let mut args = HashMap::new();

        if let Some(detail) = err.detail {
            args.insert("detail".to_string(), detail);
        }

        ServerEvent {
            event: Some(ServerEventKind::Error(
                ApplicationErrorEvent {
                    key: err.text_key.to_string(),
                    args,
                },
            )),
        }
    }
}

impl TryFrom<ServerEvent> for ApplicationError {
    type Error = ();

    fn try_from(event: ServerEvent) -> Result<Self, Self::Error> {
        match event.event {
            Some(ServerEventKind::Error(err)) => {
                let text_key = err
                    .key
                    .parse()
                    .unwrap_or(TextKeys::ConnectionErrorMessage);

                let detail = err.args.get("detail").cloned();

                Ok(ApplicationError {
                    text_key,
                    detail,
                })
            }

            _ => Err(()),
        }
    }
}

impl From<ApplicationErrorEvent> for ApplicationError {
    fn from(err: ApplicationErrorEvent) -> Self {
        let text_key = err.key.parse().unwrap_or(TextKeys::ConnectionErrorMessage);
        let detail = err.args.get("detail").cloned();
        ApplicationError { text_key, detail }
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use tonic::Status;
    use i18n_loader::TextKeys;
    use crate::domain::errors::ApplicationError;
    use crate::kdrive::{ApplicationErrorEvent, ServerEvent};
    use crate::kdrive::server_event::Event as ServerEventKind;

    #[test]
    fn application_error_grpc_round_trip_preserves_key() {
        let original = ApplicationError {
            text_key: TextKeys::TokenRequestFailed,
            detail: Some("some detail".to_string()),
        };

        let status: Status = original.clone().into();
        let decoded: ApplicationError = status.into();

        assert_eq!(decoded.text_key, original.text_key);
    }

    #[test]
    fn application_error_grpc_round_trip_preserves_detail_if_valid() {
        let original = ApplicationError {
            text_key: TextKeys::TokenRequestFailed,
            detail: Some("simple-ascii-detail".to_string()),
        };

        let status: Status = original.clone().into();
        let decoded: ApplicationError = status.into();

        assert_eq!(decoded.detail, original.detail);
    }

    #[test]
    fn application_error_grpc_drops_invalid_detail_but_keeps_key() {
        let original = ApplicationError {
            text_key: TextKeys::TokenRequestFailed,
            detail: Some("🚀 unicode not allowed in grpc metadata".to_string()),
        };

        let status: Status = original.clone().into();
        let decoded: ApplicationError = status.into();

        assert_eq!(decoded.text_key, original.text_key);
        assert!(decoded.detail.is_none());
    }

    #[test]
    fn application_error_into_server_event_preserves_key() {
        let err = ApplicationError {
            text_key: TextKeys::TokenRequestFailed,
            detail: None,
        };

        let event: ServerEvent = err.into();

        match event.event {
            Some(ServerEventKind::Error(error_event)) => {
                assert_eq!(
                    error_event.key,
                    TextKeys::TokenRequestFailed.to_string()
                );
            }
            _ => panic!("expected ServerEventKind::Error"),
        }
    }

    #[test]
    fn application_error_into_server_event_includes_detail() {
        let err = ApplicationError {
            text_key: TextKeys::TokenRequestFailed,
            detail: Some("some detail".to_string()),
        };

        let event: ServerEvent = err.into();

        match event.event {
            Some(ServerEventKind::Error(error_event)) => {
                assert_eq!(
                    error_event.args.get("detail"),
                    Some(&"some detail".to_string())
                );
            }
            _ => panic!("expected ServerEventKind::Error"),
        }
    }

    #[test]
    fn application_error_into_server_event_without_detail_has_empty_args() {
        let err = ApplicationError {
            text_key: TextKeys::TokenRequestFailed,
            detail: None,
        };

        let event: ServerEvent = err.into();

        match event.event {
            Some(ServerEventKind::Error(error_event)) => {
                assert!(error_event.args.is_empty());
            }
            _ => panic!("expected ServerEventKind::Error"),
        }
    }

    #[test]
    fn server_event_into_application_error_preserves_key() {
        let event = ServerEvent {
            event: Some(ServerEventKind::Error(
                ApplicationErrorEvent {
                    key: TextKeys::TokenRequestFailed.to_string(),
                    args: HashMap::new(),
                },
            )),
        };

        let err = ApplicationError::try_from(event).expect("conversion should succeed");

        assert_eq!(err.text_key, TextKeys::TokenRequestFailed);
    }

    #[test]
    fn server_event_into_application_error_includes_detail() {
        let mut args = HashMap::new();
        args.insert("detail".to_string(), "some detail".to_string());

        let event = ServerEvent {
            event: Some(ServerEventKind::Error(
                ApplicationErrorEvent {
                    key: TextKeys::TokenRequestFailed.to_string(),
                    args,
                },
            )),
        };

        let err = ApplicationError::try_from(event).expect("conversion should succeed");

        assert_eq!(err.detail, Some("some detail".to_string()));
    }

    #[test]
    fn server_event_into_application_error_unknown_key_falls_back() {
        let event = ServerEvent {
            event: Some(ServerEventKind::Error(
                ApplicationErrorEvent {
                    key: "UnknownKey".to_string(),
                    args: HashMap::new(),
                },
            )),
        };

        let err = ApplicationError::try_from(event).expect("conversion should succeed");

        assert_eq!(
            err.text_key,
            TextKeys::ConnectionErrorMessage
        );
    }

    #[test]
    fn non_error_server_event_cannot_be_converted() {
        let event = ServerEvent {
            event: None,
        };

        let result = ApplicationError::try_from(event);

        assert!(result.is_err());
    }
}