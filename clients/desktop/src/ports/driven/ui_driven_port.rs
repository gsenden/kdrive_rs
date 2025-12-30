use common::domain::errors::ApplicationError;

pub trait UIDrivenPort {
    fn show_login_view(&self);
    fn show_error_view(&self, error: ApplicationError);
    fn show_home_view(&self);
    fn show_loading_view(&self);
}
