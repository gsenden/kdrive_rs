use crate::domain::errors::ConfigurationError;
use crate::domain::tokens::Tokens;

pub trait TokenStoreDrivenPort {
    fn is_available(&self) -> bool;
    fn load(&self) -> Result<Option<Tokens>, ConfigurationError>;
    fn save(&self, tokens: &Tokens) -> Result<(), ConfigurationError>;
}