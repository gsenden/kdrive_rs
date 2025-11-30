use crate::domain::errors::EventBusError;
use crate::domain::events::EngineEvent;

pub trait EventBusDrivenPort {
    fn emit(&self, event: EngineEvent) -> Result<(), EventBusError>;
}