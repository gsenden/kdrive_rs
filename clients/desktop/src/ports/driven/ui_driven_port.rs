use common::domain::errors::ApplicationError;

pub trait UIDrivenPort {
    fn show_login_view(&mut self, url: String);
    fn show_error_view(&mut self, error: ApplicationError);
    fn show_home_view(&mut self);
    fn show_loading_view(&mut self);
}
