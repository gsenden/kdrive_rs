use common::domain::errors::ApplicationError;
use crate::domain::events::ServerEventStream;

pub trait ServerDrivenPort: Send + Sync {
    fn is_authenticated(&self) -> impl Future<Output = Result<bool, ApplicationError>> + Send;
    fn start_initial_auth_flow(&self) -> impl Future<Output = Result<String, ApplicationError>> + Send;
    fn continue_initial_auth_flow(&self) -> impl Future<Output = Result<(), ApplicationError>> + Send;
    fn subscribe_events(&self) -> impl Future<Output = Result<ServerEventStream, ApplicationError>> + Send;
}