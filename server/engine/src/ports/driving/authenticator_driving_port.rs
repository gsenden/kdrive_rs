pub trait AuthenticatorDrivingPort {
    fn is_authenticated(&self) -> bool;
}