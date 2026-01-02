use common::domain::errors::ApplicationError;
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
    pub event_bus: EventPort,
}

impl<AuthPort, TokenPort, EventPort> Engine<AuthPort, TokenPort, EventPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort,
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

    pub async fn start_initial_auth_flow(&mut self) -> Result<String, ApplicationError> {
        self.authenticator_driven_port.start_initial_auth_flow().await
    }

    pub async fn continue_initial_auth_flow(&mut self) {
        let result = self.do_auth_flow().await;

        let event = match result {
            Ok(()) => EngineEvent::AuthFlowCompleted,
            Err(error) => EngineEvent::AuthFlowFailed { reason: error },
        };

        let _ = self.event_bus.emit(event);
    }

    async fn do_auth_flow(&mut self) -> Result<(), ApplicationError> {
        self.authenticator_driven_port.continue_initial_auth_flow().await?;
        let tokens = self.authenticator_driven_port.get_tokens().await?;
        self.token_store.save_tokens(&tokens)?;
        Ok(())
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
    use crate::domain::engine::Engine;
    use crate::domain::events::EngineEvent;
    use crate::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
    use crate::domain::test_helpers::fake_event_bus::FakeEventBus;
    use crate::domain::test_helpers::fake_token_store_adapter::{FakeTokenStoreFileAdapter, FakeTokenStoreRingAdapter};
    use crate::domain::test_helpers::fake_token_store::FakeTokenStore;
    use crate::domain::tokens::TokenStore;
    use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;

    pub struct TestEngineBuilder {
        auth: FakeAuthenticatorDrivenAdapter,
        token_store: FakeTokenStore,
        event_bus: FakeEventBus,
    }

    impl TestEngineBuilder {
        pub fn new() -> Self {
            Self {
                auth: FakeAuthenticatorDrivenAdapter::new_default(),
                token_store: TokenStore::load(
                    Some(FakeTokenStoreRingAdapter::with_tokens()),
                    None
                ).unwrap(),
                event_bus: FakeEventBus::new()
            }
        }

        pub fn with_auth(mut self, auth: FakeAuthenticatorDrivenAdapter) -> Self {
            self.auth = auth;
            self
        }
        pub fn with_token_store(mut self, token_store: FakeTokenStore) -> Self {
            self.token_store = token_store;
            self
        }

        pub fn with_empty_token_store(mut self) -> Self {
            self.token_store = TokenStore::load(
                Some(FakeTokenStoreRingAdapter::empty()),
                None
            ).unwrap();
            self
        }

        pub fn build(self) -> Engine<FakeAuthenticatorDrivenAdapter, FakeTokenStore, FakeEventBus>
        {
            Engine::new(
                self.auth,
                self.token_store,
                self.event_bus
            )
        }
    }


    #[test]
    fn engine_is_not_authenticated_when_token_store_has_no_tokens() {
        // Given an unauthenticated engine with a token store with no tokens
        let engine = TestEngineBuilder::new()
            .with_empty_token_store()
            .build();

        // When is_authenticated is called
        let result = engine.is_authenticated();

        // Then it returns false
        assert_eq!(result, false);
    }

    #[test]
    fn engine_is_authenticated_when_token_store_has_tokens() {
        // Given an engine with a token store that has tokens
        let engine = TestEngineBuilder::new()
            .build();

        // When is_authenticated is called
        let result = engine.is_authenticated();

        // Then it returns true
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn engine_returns_auth_url_when_starting_auth_flow() {
        // Given an unauthenticated engine
        let mut engine = TestEngineBuilder::new()
            .build();

        // When start_initial_auth_flow is called
        let result = engine.start_initial_auth_flow().await;

        // Then it returns a valid auth URL
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn engine_can_complete_full_auth_flow() {
        // Given an unauthenticated engine with a token store with no tokens
        let mut engine = TestEngineBuilder::new()
            .with_empty_token_store()
            .build();

        // When start_initial_auth_flow is called,
        // Error is ignored here because in reality a user would authenticate using the browser
        // what is needed is that the auth flow is completed
        _ = engine.start_initial_auth_flow().await;

        // And continue_initial_auth_flow is called
        engine.continue_initial_auth_flow().await;

        // Then both succeed
        assert!(engine.event_bus.get_events().contains(
            &EngineEvent::AuthFlowCompleted
        ));
    }

    #[tokio::test]
    async fn engine_emits_tokens_stored_event_when_completing_auth_flow() {
        // Given an engine with event bus
        let mut engine = TestEngineBuilder::new()
            .build();

        // When continue_initial_auth_flow is called
        _ = engine.continue_initial_auth_flow().await;

        // Then TokensStored event is emitted
        assert!(engine.event_bus.get_events().contains(&crate::domain::events::EngineEvent::AuthFlowCompleted));
    }

    #[tokio::test]
    async fn engine_emits_auth_flow_failed_event_when_auth_fails() {
        // Given an engine that will fail auth
        let adapter = FakeAuthenticatorDrivenAdapter::new_default_failing();
        let mut engine = TestEngineBuilder::new()
            .with_auth(adapter)
            .build();

        // When continue_initial_auth_flow fails
       _  = engine.continue_initial_auth_flow().await;


        // Then AuthFlowFailed event is emitted
        assert!(engine.event_bus.get_events().iter().any(|e|
            matches!(e, EngineEvent::AuthFlowFailed { .. })
        ));
    }

    #[tokio::test]
    async fn engine_persists_tokens_after_auth_flow() {
        // Given an engine with token store
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let fake_ring_tokens = FakeTokenStoreRingAdapter::empty();
        let fake_file_tokens = FakeTokenStoreFileAdapter::empty();
        let token_store: FakeTokenStore = FakeTokenStore::load(
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