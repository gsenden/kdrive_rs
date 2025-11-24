pub trait AuthenticatorDrivingPort {
    fn is_connected(&self) -> bool;
}