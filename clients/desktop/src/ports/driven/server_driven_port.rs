use crate::domain::errors::ClientError;

pub trait ServerDrivenPort: Send + Sync {
    fn is_authenticated(&self) -> impl Future<Output = Result<bool, ClientError>> + Send;
    fn start_initial_auth_flow(&self) -> impl Future<Output = Result<String, ClientError>> + Send;
}