use common::domain::errors::ApplicationError;
#[derive(Debug, PartialEq, Clone)]
pub enum View {
    Home,
    Login,
    Error(ApplicationError),
}