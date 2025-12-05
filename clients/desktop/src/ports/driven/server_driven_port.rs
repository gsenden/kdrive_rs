use crate::domain::errors::ClientError;

pub trait ServerDrivenPort: Send + Sync {
    fn is_authenticated(&self) -> impl Future<Output = Result<bool, ClientError>> + Send;
}