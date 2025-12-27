use common::domain::errors::ApplicationError;
use crate::domain::events::EngineEvent;

pub trait EventBusDrivenPort {
    fn emit(&self, event: EngineEvent) -> Result<(), ApplicationError>;
}