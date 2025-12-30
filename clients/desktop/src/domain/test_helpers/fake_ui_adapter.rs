use std::sync::{Arc, Mutex};
use common::domain::errors::ApplicationError;
use crate::ports::driven::ui_driven_port::UIDrivenPort;

#[derive(Clone)]
pub struct FakeUIAdapter {
    login_view_shown: Arc<Mutex<bool>>,
    error: Arc<Mutex<Option<ApplicationError>>>,
    home_view_shown: Arc<Mutex<bool>>,
    loading_view_shown: Arc<Mutex<bool>>,
}

impl FakeUIAdapter {
    pub fn new() -> Self {
        Self {
            login_view_shown: Arc::new(Mutex::new(false)),
            error: Arc::new(Mutex::new(None)),
            home_view_shown: Arc::new(Mutex::new(false)),
            loading_view_shown: Arc::new(Mutex::new(false)),
        }
    }

    pub fn login_view_was_shown(&self) -> bool {
        *self.login_view_shown.lock().unwrap()
    }

    pub fn error_view_was_shown(&self) -> bool {
        self.error.lock().unwrap().is_some()
    }

    pub fn get_error(&self) -> Option<ApplicationError> {
        self.error.lock().unwrap().clone()
    }

    pub fn home_view_was_shown(&self) -> bool {
        *self.home_view_shown.lock().unwrap()
    }

    pub fn loading_view_was_shown(&self) -> bool {
        *self.loading_view_shown.lock().unwrap()
    }
}

impl UIDrivenPort for FakeUIAdapter {
    fn show_login_view(&self) {
        *self.login_view_shown.lock().unwrap() = true;
    }

    fn show_error_view(&self, error: ApplicationError) {
        *self.error.lock().unwrap() = Some(error);
    }

    fn show_home_view(&self) {
        *self.home_view_shown.lock().unwrap() = true;
    }

    fn show_loading_view(&self) {
        *self.loading_view_shown.lock().unwrap() = true;
    }
}