mod app;
mod tauri_commands;
mod frontend_errors;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
