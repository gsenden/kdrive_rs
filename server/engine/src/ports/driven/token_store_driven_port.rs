use common::domain::errors::ApplicationError;
use crate::domain::tokens::Tokens;

pub trait TokenStoreDrivenPort {
    fn is_available(&self) -> bool;
    fn load(&self) -> Result<Option<Tokens>, ApplicationError>;
    fn save(&self, tokens: &Tokens) -> Result<(), ApplicationError>;
}