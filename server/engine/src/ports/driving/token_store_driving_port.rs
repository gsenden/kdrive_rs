use common::domain::errors::ApplicationError;
use crate::domain::tokens::Tokens;

pub trait TokenStoreDrivingPort {
    fn has_tokens(&self) -> bool;
    fn access_token(&self) -> Option<&str>;
    fn refresh_token(&self) -> Option<&str>;
    fn expires_at(&self) -> Option<i64>;
    fn save_tokens(&mut self, tokens: &Tokens) -> Result<(), ApplicationError>;
}