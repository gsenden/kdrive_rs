use crate::domain::errors::ServerError;
use crate::domain::tokens::Tokens;

pub trait TokenStoreDrivenPort {
    fn is_available(&self) -> bool;
    fn load(&self) -> Result<Option<Tokens>, ServerError>;
    fn save(&self, tokens: &Tokens) -> Result<(), ServerError>;
}