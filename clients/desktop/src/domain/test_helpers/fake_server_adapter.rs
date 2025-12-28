use common::domain::errors::ApplicationError;
use crate::ports::driven::server_driven_port::ServerDrivenPort;

pub const TEST_URL_RESPONSE: &str = "http://localhost:8080/test-url-response";

#[derive(PartialEq)]
#[derive(Clone)]
pub struct FakeServerAdapter {
    authenticated: bool,
    error: Option<ApplicationError>,
}

impl FakeServerAdapter {

    pub fn new(authenticated: bool) -> Self {
        FakeServerAdapter {authenticated, error: None}
    }

    pub fn set_error(&mut self, error: ApplicationError) {
        self.error = Some(error);
    }

}

impl ServerDrivenPort for FakeServerAdapter {
    async fn is_authenticated(&self) -> Result<bool, ApplicationError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        } else {
            Ok(self.authenticated)
        }
    }

    async fn start_initial_auth_flow(&self) -> Result<String, ApplicationError> {
        if let Some(error) = &self.error {
            Err(error.clone())
        } else {
            Ok(TEST_URL_RESPONSE.to_string())

        }
    }

    async fn continue_initial_auth_flow(&self) -> Result<(), ApplicationError> {
        Ok(())
    }
}