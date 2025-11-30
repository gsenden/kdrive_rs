use crate::domain::events::EngineEvent;
use crate::ports::driven::event_bus_driven_port::EventBusDrivenPort;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FakeEventBus {
    events: Arc<Mutex<Vec<EngineEvent>>>,
}

impl FakeEventBus {
    pub fn new() -> Self {
        FakeEventBus {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_events(&self) -> Vec<EngineEvent> {
        self.events.lock().unwrap().clone()
    }
}

impl EventBusDrivenPort for FakeEventBus {
    fn emit(&self, event: EngineEvent) {
        self.events.lock().unwrap().push(event);
    }
}