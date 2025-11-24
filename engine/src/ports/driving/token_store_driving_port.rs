pub trait TokenStoreDrivingPort {
    fn access_token(&self) -> String;
    fn refresh_token(&self) -> String;
    fn expires_at(&self) -> i64;
}

