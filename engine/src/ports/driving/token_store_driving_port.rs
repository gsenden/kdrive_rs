pub trait TokenStoreDrivingPort {
    fn has_tokens(&self) -> bool;
    fn access_token(&self) -> Option<&str>;
    fn refresh_token(&self) -> Option<&str>;
    fn expires_at(&self) -> Option<i64>;
}