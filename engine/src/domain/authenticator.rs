use crate::domain::errors::AuthFlowError;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;

pub struct Authenticator<AP: AuthenticatorDrivenPort> {
    authenticator_port: AP,
    is_connected: bool,
}

impl <AP: AuthenticatorDrivenPort> Authenticator<AP> {
    pub fn new(authenticator_port: AP) -> Self {
        Authenticator { authenticator_port, is_connected: false }
    }

    pub async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        self.authenticator_port.start_initial_auth_flow().await
    }

    pub async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        self.authenticator_port.continue_initial_auth_flow().await
    }
}

impl<AP: AuthenticatorDrivenPort> AuthenticatorDrivingPort for Authenticator<AP> {
    fn is_connected(&self) -> bool {
        self.is_connected
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::authenticator::Authenticator;
    use crate::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
    use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;

    #[test]
    fn when_authenticator_is_created_without_previous_connection_it_is_not_connected() {
        // Given a new authenticator-driven adapter
        let adapter = FakeAuthenticatorDrivenAdapter::new_default();

        // When the authenticator is created
        let authenticator = Authenticator::new(adapter);

        // Then the authenticator is not connected
        assert_eq!(authenticator.is_connected(), false);
    }
}