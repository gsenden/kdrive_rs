pub trait TokenStoreDrivingPort {
    fn access_token(&self) -> &str;
    fn refresh_token(&self) -> &str;
    fn expires_at(&self) -> i64;
}

