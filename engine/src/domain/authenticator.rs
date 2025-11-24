use crate::domain::errors::AuthFlowError;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;

pub struct Authenticator<AP: AuthenticatorDrivenPort> {
    authenticator_port: AP
}

impl <AP: AuthenticatorDrivenPort> Authenticator<AP> {
    pub fn new(authenticator_port: AP) -> Self { Authenticator { authenticator_port }}

    pub async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        self.authenticator_port.start_initial_auth_flow().await
    }

    pub async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        self.authenticator_port.continue_initial_auth_flow().await
    }

}