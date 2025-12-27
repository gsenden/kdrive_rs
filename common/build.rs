fn main() {
    if let Err(e) = i18n_loader::load() {
        eprintln!("Build failed due to i18n configuration errors: {}", e);
        std::process::exit(1);
    }
}