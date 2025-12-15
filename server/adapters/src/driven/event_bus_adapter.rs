use std::sync::{Arc, Mutex};
use engine::domain::errors::ServerError;
use engine::domain::events::EngineEvent;
use engine::ports::driven::event_bus_driven_port::EventBusDrivenPort;

#[derive(Clone)]
pub struct EventBusAdapter {
    events: Arc<Mutex<Vec<EngineEvent>>>,
}

impl EventBusAdapter {
    pub fn new() -> Self {
        EventBusAdapter {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_events(&self) -> Vec<EngineEvent> {
        self.events
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_default()
    }
}

impl EventBusDrivenPort for EventBusAdapter {
    fn emit(&self, event: EngineEvent) -> Result<(), ServerError> {
        self.events
            .lock()
            .map_err(|e| ServerError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )))?
            .push(event);
        Ok(())
    }
}

#[cfg(test)]
// adapters/src/driven/event_bus_adapter_test.rs
mod tests {
    use engine::domain::events::EngineEvent;
    use crate::driven::event_bus_adapter::EventBusAdapter;
    use engine::ports::driven::event_bus_driven_port::EventBusDrivenPort;

    #[test]
    fn event_bus_adapter_can_receive_event() {
        // Given an EventBusAdapter
        let adapter = EventBusAdapter::new();

        // When we emit an AuthFlowCompleted event
        adapter.emit(EngineEvent::AuthFlowCompleted).unwrap();

        // Then we can retrieve the event
        let events = adapter.get_events();
        assert!(events.contains(&EngineEvent::AuthFlowCompleted));
    }

    #[test]
    fn event_bus_adapter_handles_concurrent_access() {
        // Given an EventBusAdapter
        let adapter = EventBusAdapter::new();

        // When multiple threads try to emit events concurrently
        let mut handles = vec![];
        for i in 0..10 {
            let adapter_clone = adapter.clone();
            let handle = std::thread::spawn(move || {
                let event = if i % 2 == 0 {
                    EngineEvent::AuthFlowCompleted
                } else {
                    EngineEvent::AuthFlowFailed {
                        reason: format!("error {}", i)
                    }
                };
                adapter_clone.emit(event).unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Then all events are stored safely
        let events = adapter.get_events();
        assert_eq!(events.len(), 10);
    }


}