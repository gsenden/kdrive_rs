use crate::domain::events::EngineEvent;

pub trait EventBusDrivenPort {
    fn emit(&self, event: EngineEvent);
}