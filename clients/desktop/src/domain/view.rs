use crate::domain::errors::ClientError;

#[derive(Debug, PartialEq, Clone)]
pub enum View {
    Home,
    Login,
    Error(ClientError),
}