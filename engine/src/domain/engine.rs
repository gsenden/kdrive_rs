use crate::domain::errors::AuthFlowError;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;

pub struct Engine<AP: AuthenticatorDrivenPort> {
    authenticator_driven_port: AP,
    is_authenticated: bool,
}

impl <AP: AuthenticatorDrivenPort> Engine<AP> {
    pub fn new(authenticator_port: AP) -> Self {
        Engine { authenticator_driven_port: authenticator_port, is_authenticated: false }
    }

    pub async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        self.authenticator_driven_port.start_initial_auth_flow().await
    }

    pub async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        self.authenticator_driven_port.continue_initial_auth_flow().await
    }
}

impl<AP: AuthenticatorDrivenPort> AuthenticatorDrivingPort for Engine<AP> {
    fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::engine::Engine;
    use crate::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
    use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;

    #[test]
    fn when_engine_is_created_without_previous_connection_it_is_not_connected() {
        // Given a new authenticator-driven adapter
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();

        // When the authenticator is created
        let authenticator = Engine::new(adapter);

        // Then the authenticator is not connected
        assert_eq!(authenticator.is_authenticated(), false);
    }

    #[test]
    fn engine_without_stored_tokens_is_not_authenticated() {
        // Given an engine without stored tokens
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();
        let engine = Engine::new(adapter);

        // When is_authenticated is called
        let result = engine.is_authenticated();

        // Then it returns false
        assert_eq!(result, false);
    }
}