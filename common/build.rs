use std::{fs, path::Path};

fn main() {
    let i18n_dir = Path::new("i18n");

    for lang_dir in fs::read_dir(i18n_dir).unwrap() {
        let lang_dir = lang_dir.unwrap();
        let path = lang_dir.path();

        if !path.is_dir() {
            continue;
        }

        let ftl = path.join("app.ftl");
        fs::read_to_string(&ftl)
            .expect("Missing app.ftl");

    }
}