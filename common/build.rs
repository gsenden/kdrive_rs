fn main() {
    if let Err(err) = tonic_prost_build::compile_protos("kdrive.proto") {
        eprintln!("Build failed due to proto build error(s): {}", err);
        std::process::exit(1);
    }
    if let Err(e) = i18n_loader::load() {
        eprintln!("Build failed due to i18n configuration error(s): {}", e);
        std::process::exit(1);
    }

}