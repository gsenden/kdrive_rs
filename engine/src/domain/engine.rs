use crate::domain::errors::AuthFlowError;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;
use crate::ports::driving::token_store_driving_port::TokenStoreDrivingPort;

pub struct Engine<AuthPort, TokenPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
{
    authenticator_driven_port: AuthPort,
    #[allow(dead_code)]
    token_store: TokenPort,
}

impl<AuthPort, TokenPort> Engine<AuthPort, TokenPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
{
    pub fn new(
        authenticator_port: AuthPort,
        token_store: TokenPort,
    ) -> Self {
        Engine {
            authenticator_driven_port: authenticator_port,
            token_store,
        }

    }

    pub async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        self.authenticator_driven_port.start_initial_auth_flow().await
    }

    pub async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        self.authenticator_driven_port.continue_initial_auth_flow().await
    }
}

impl<AuthPort, TokenPort> AuthenticatorDrivingPort for Engine<AuthPort, TokenPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
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
    use crate::domain::test_helpers::fake_token_store_adapter::FakeTokenStoreRingAdapter;
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
        let engine = Engine::new(adapter, token_store);

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
        let engine = Engine::new(adapter, token_store);

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
        let mut engine = Engine::new(adapter, token_store);

        // When start_initial_auth_flow is called
        let result = engine.start_initial_auth_flow().await;

        // Then it returns a valid auth URL
        assert!(result.is_ok());
    }

}