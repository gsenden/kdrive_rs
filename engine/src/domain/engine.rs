use crate::domain::errors::AuthFlowError;
use crate::domain::events::EngineEvent;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driven::event_bus_driven_port::EventBusDrivenPort;
use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;
use crate::ports::driving::token_store_driving_port::TokenStoreDrivingPort;

pub struct Engine<AuthPort, TokenPort, EventPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort
{
    authenticator_driven_port: AuthPort,
    #[allow(dead_code)]
    token_store: TokenPort,
    event_bus: EventPort,
}

impl<AuthPort, TokenPort, EventPort> Engine<AuthPort, TokenPort, EventPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort
{
    pub fn new(
        authenticator_port: AuthPort,
        token_store: TokenPort,
        event_bus: EventPort
    ) -> Self {
        Engine {
            authenticator_driven_port: authenticator_port,
            token_store,
            event_bus
        }

    }

    pub async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        self.authenticator_driven_port.start_initial_auth_flow().await
    }

    pub async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        let result = self.authenticator_driven_port.continue_initial_auth_flow().await;
        match result {
            Ok(result) => {
                let tokens = self.authenticator_driven_port.get_tokens().await?;
                self.token_store.save_tokens(&tokens)?;
                self.event_bus.emit(EngineEvent::AuthFlowCompleted)?;
                Ok(result)
            }
            Err(error) => {
                self.event_bus.emit(EngineEvent::AuthFlowFailed {reason: error.to_string()})?;
                Err(error)
            }
        }
    }
}

impl<AuthPort, TokenPort, EventPort> AuthenticatorDrivingPort for Engine<AuthPort, TokenPort, EventPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort
{
    fn is_authenticated(&self) -> bool {
        self.token_store.has_tokens()
    }
}

#[cfg(test)]
mod tests {
    // use crate::domain::test_helpers::fake_token_store_adapter::FakeTokenStoreRingAdapter;
    // use crate::domain::tokens::TokenStore;

    use crate::domain::engine::Engine;
    use crate::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
    use crate::domain::test_helpers::fake_event_bus::FakeEventBus;
    use crate::domain::test_helpers::fake_token_store_adapter::{FakeTokenStoreFileAdapter, FakeTokenStoreRingAdapter};
    use crate::domain::test_helpers::test_store::TestStore;
    use crate::domain::tokens::TokenStore;
    use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;

    #[test]
    fn engine_is_not_authenticated_when_token_store_has_no_tokens() {
        // Given an engine with an empty token store
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let token_store: TestStore = TokenStore::load(
            Some(FakeTokenStoreRingAdapter::empty()),
            None
        ).unwrap();
        let event_bus = FakeEventBus::new();
        let engine = Engine::new(adapter, token_store, event_bus);

        // When is_authenticated is called
        let result = engine.is_authenticated();

        // Then it returns false
        assert_eq!(result, false);
    }

    #[test]
    fn engine_is_authenticated_when_token_store_has_tokens() {
        // Given an engine with a token store that has tokens
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let token_store: TestStore = TokenStore::load(
            Some(FakeTokenStoreRingAdapter::with_tokens()),
            None
        ).unwrap();
        let event_bus = FakeEventBus::new();
        let engine = Engine::new(adapter, token_store, event_bus);

        // When is_authenticated is called
        let result = engine.is_authenticated();

        // Then it returns true
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn engine_returns_auth_url_when_starting_auth_flow() {
        // Given an unauthenticated engine
        use crate::domain::test_helpers::test_store::TestStore;

        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let token_store: TestStore = TokenStore::load(
            Some(FakeTokenStoreRingAdapter::empty()),
            None
        ).unwrap();
        let event_bus = FakeEventBus::new();
        let mut engine = Engine::new(adapter, token_store, event_bus);

        // When start_initial_auth_flow is called
        let result = engine.start_initial_auth_flow().await;

        // Then it returns a valid auth URL
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn engine_can_complete_full_auth_flow() {
        // Given an unauthenticated engine
        use crate::domain::test_helpers::test_store::TestStore;

        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let token_store: TestStore = TokenStore::load(
            Some(FakeTokenStoreRingAdapter::empty()),
            None
        ).unwrap();
        let event_bus = FakeEventBus::new();
        let mut engine = Engine::new(adapter, token_store, event_bus);

        // When start_initial_auth_flow is called
        _ = engine.start_initial_auth_flow().await;

        // And continue_initial_auth_flow is called
        let continue_result = engine.continue_initial_auth_flow().await;

        // Then both succeed
        assert!(continue_result.is_ok());
    }

    #[tokio::test]
    async fn engine_emits_tokens_stored_event_when_completing_auth_flow() {
        // Given an engine with event bus
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let token_store: TestStore = TokenStore::load(
            Some(FakeTokenStoreRingAdapter::empty()),
            None
        ).unwrap();
        let event_bus = FakeEventBus::new();
        let mut engine = Engine::new(adapter, token_store, event_bus.clone());

        // When continue_initial_auth_flow is called
        _ = engine.continue_initial_auth_flow().await;

        // Then TokensStored event is emitted
        assert!(event_bus.get_events().contains(&crate::domain::events::EngineEvent::AuthFlowCompleted));
    }

    #[tokio::test]
    async fn engine_emits_auth_flow_failed_event_when_auth_fails() {
        // Given an engine that will fail auth
        let adapter = FakeAuthenticatorDrivenAdapter::new_default_failing();
        let token_store: TestStore = TokenStore::load(
            Some(FakeTokenStoreRingAdapter::empty()),
            None
        ).unwrap();
        let event_bus = FakeEventBus::new();
        let mut engine = Engine::new(adapter, token_store, event_bus.clone());

        // When continue_initial_auth_flow fails
        _ = engine.continue_initial_auth_flow().await;

        // Then AuthFlowFailed event is emitted
        assert!(event_bus.get_events().iter().any(|e|
            matches!(e, crate::domain::events::EngineEvent::AuthFlowFailed { .. })
        ));
    }

    #[tokio::test]
    async fn engine_persists_tokens_after_auth_flow() {
        // Given an engine with token store
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let fake_ring_tokens = FakeTokenStoreRingAdapter::empty();
        let fake_file_tokens = FakeTokenStoreFileAdapter::empty();
        let token_store: TestStore = TestStore::load(
            Some(fake_ring_tokens),
            Some(fake_file_tokens)
        ).unwrap();
        let event_bus = FakeEventBus::new();
        let mut engine = Engine::new(adapter, token_store, event_bus);

        // When auth flow completes
        _ = engine.start_initial_auth_flow().await;
        _ = engine.continue_initial_auth_flow().await;

        // Then tokens are persisted
        assert!(engine.is_authenticated());
    }

}