use common::domain::errors::ApplicationError;

pub trait ServerDrivenPort: Send + Sync {
    fn is_authenticated(&self) -> impl Future<Output = Result<bool, ApplicationError>> + Send;
    fn start_initial_auth_flow(&self) -> impl Future<Output = Result<String, ApplicationError>> + Send;
}