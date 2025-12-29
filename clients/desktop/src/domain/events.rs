use std::pin::Pin;
use futures_core::Stream;
use common::domain::errors::ApplicationError;
use common::kdrive::ServerEvent;

pub type ServerEventStream = Pin<Box<dyn Stream<Item = Result<ServerEvent, ApplicationError>> + Send>>;