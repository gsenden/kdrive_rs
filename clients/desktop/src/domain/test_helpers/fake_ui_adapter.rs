use std::sync::{Arc, Mutex};
use common::domain::errors::ApplicationError;
use crate::ports::driven::ui_driven_port::UIDrivenPort;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FakeUIAdapter {
    login_view_shown: Arc<Mutex<bool>>,
    error: Arc<Mutex<Option<ApplicationError>>>,
    home_view_shown: Arc<Mutex<bool>>,
    loading_view_shown: Arc<Mutex<bool>>,
    login_url: Arc<Mutex<Option<String>>>,
}

#[allow(dead_code)]
impl FakeUIAdapter {
    pub fn new() -> Self {
        Self {
            login_view_shown: Arc::new(Mutex::new(false)),
            error: Arc::new(Mutex::new(None)),
            home_view_shown: Arc::new(Mutex::new(false)),
            loading_view_shown: Arc::new(Mutex::new(false)),
            login_url: Arc::new(Mutex::new(None)),
        }
    }

    pub fn login_view_was_shown(&self) -> bool {
        *self.login_view_shown.lock().unwrap()
    }

    pub fn error_view_was_shown(&self) -> bool {
        self.error.lock().unwrap().is_some()
    }

    pub fn home_view_was_shown(&self) -> bool {
        *self.home_view_shown.lock().unwrap()
    }

    pub fn loading_view_was_shown(&self) -> bool {
        *self.loading_view_shown.lock().unwrap()
    }

    pub fn login_url_shown(&self) -> Option<String> { self.login_url.lock().unwrap().clone() }
}

impl UIDrivenPort for FakeUIAdapter {
    fn show_login_view(&mut self, url: String) {
        *self.login_view_shown.lock().unwrap() = true;
        *self.login_url.lock().unwrap() = Some(url);
    }

    fn show_error_view(&mut self, error: ApplicationError) {
        *self.error.lock().unwrap() = Some(error);
    }

    fn show_home_view(&mut self) {
        *self.home_view_shown.lock().unwrap() = true;
    }

    fn show_loading_view(&mut self) {
        *self.loading_view_shown.lock().unwrap() = true;
    }
}