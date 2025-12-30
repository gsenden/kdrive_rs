use dioxus::prelude::*;
use common::domain::errors::ApplicationError;
use common::ports::i18n_driven_port::I18nDrivenPort;
use crate::ports::driven::ui_driven_port::UIDrivenPort;
use crate::ui::views::{Login, Home, ErrorView, ConnectingView};

#[derive(Clone)]
pub struct DioxusAdapter<I18nPort: I18nDrivenPort> {
    current_element: Signal<Element>,
    i18n: I18nPort
}

impl<I18nPort: I18nDrivenPort> DioxusAdapter<I18nPort> {
    pub fn new(initial: Signal<Element>, i18n: I18nPort) -> Self {
        Self { current_element: initial, i18n }
    }

    // pub fn current_element(&self) -> Element {
    //     (self.current_element)()
    // }
}

impl<I18n: I18nDrivenPort + Clone + 'static> UIDrivenPort for DioxusAdapter<I18n> {
    fn show_login_view(&mut self, url: String) {
        let i18n = self.i18n.clone();
        self.current_element.set(rsx! { Login { i18n, url } });
    }

    fn show_error_view(&mut self, error: ApplicationError) {
        let i18n = self.i18n.clone();
        self.current_element.set(rsx! { ErrorView { error, i18n } });
    }

    fn show_home_view(&mut self) {
        self.current_element.set(rsx! { Home {} });
    }

    fn show_loading_view(&mut self) {
        let i18n = self.i18n.clone();
        self.current_element.set(rsx! { ConnectingView { i18n } });
    }
}