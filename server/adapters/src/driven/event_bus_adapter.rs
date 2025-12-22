use std::sync::{Arc, Mutex};
use engine::domain::errors::ServerError;
use engine::domain::events::EngineEvent;
use engine::ports::driven::event_bus_driven_port::EventBusDrivenPort;

use tokio::sync::broadcast;

#[derive(Clone)]
pub struct EventBusAdapter {
    sender: broadcast::Sender<EngineEvent>,
}

impl EventBusAdapter {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(32);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EngineEvent> {
        self.sender.subscribe()
    }
}

impl EventBusDrivenPort for EventBusAdapter {
    fn emit(&self, event: EngineEvent) -> Result<(), ServerError> {
        let _ = self.sender.send(event);
        Ok(())
    }
}
#[cfg(test)]
// adapters/src/driven/event_bus_adapter_test.rs
mod tests {
    use engine::domain::events::EngineEvent;
    use crate::driven::event_bus_adapter::EventBusAdapter;
    use engine::ports::driven::event_bus_driven_port::EventBusDrivenPort;

    #[tokio::test]
    async fn event_bus_adapter_emits_event_to_subscriber() {
        // Given
        let bus = EventBusAdapter::new();
        let mut rx = bus.subscribe();

        // When
        bus.emit(EngineEvent::AuthFlowCompleted).unwrap();

        // Then
        let received = rx.recv().await.unwrap();
        assert_eq!(received, EngineEvent::AuthFlowCompleted);
    }

    #[tokio::test]
    async fn event_bus_adapter_handles_concurrent_access() {
        let bus = EventBusAdapter::new();
        let mut rx = bus.subscribe();

        let mut handles = vec![];

        for i in 0..10 {
            let bus = bus.clone();
            handles.push(tokio::spawn(async move {
                let event = if i % 2 == 0 {
                    EngineEvent::AuthFlowCompleted
                } else {
                    EngineEvent::AuthFlowFailed {
                        reason: format!("error {}", i),
                    }
                };
                bus.emit(event).unwrap();
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let mut received = vec![];
        for _ in 0..10 {
            received.push(rx.recv().await.unwrap());
        }

        assert_eq!(received.len(), 10);
    }
}