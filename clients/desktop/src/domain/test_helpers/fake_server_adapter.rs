use crate::domain::errors::ClientError;
use crate::ports::driven::server_driven_port::ServerDrivenPort;

pub struct FakeServerAdapter {
    authenticated: bool,
    error: Option<ClientError>
}

impl FakeServerAdapter {
    pub fn new(authenticated: bool) -> Self {
        FakeServerAdapter {authenticated, error: None}
    }

    pub fn set_error(&mut self, error: ClientError) {
        self.error = Some(error);
    }
}

impl ServerDrivenPort for FakeServerAdapter {
    async fn is_authenticated(&self) -> Result<bool, ClientError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        } else {
            Ok(self.authenticated)
        }
    }
}