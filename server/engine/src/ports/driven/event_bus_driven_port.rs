use crate::domain::errors::ServerError;
use crate::domain::events::EngineEvent;

pub trait EventBusDrivenPort {
    fn emit(&self, event: EngineEvent) -> Result<(), ServerError>;
}