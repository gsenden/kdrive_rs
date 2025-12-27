use std::collections::HashMap;
use i18n_embed::fluent::FluentLanguageLoader;
use rust_embed::RustEmbed;
use strum::IntoEnumIterator;
pub use crate::language::Language;
use thiserror::Error;
pub use crate::text_keys::TextKeys;

pub mod language;
pub mod text_keys;

#[derive(RustEmbed)]
#[folder = "../i18n/"]
struct Localizations;

pub const DOMAIN: &str = "app";

#[derive(Debug, Error)]
pub enum I18nLoaderError {
    #[error("I18N initialization failed: {0}")]
    I18nEmbed(#[from] i18n_embed::I18nEmbedError),
    #[error("Failed to initialize I18N. In language: {lang}, missing key: {key}")]
    MissingKey { lang: String, key: String },
    #[error("Failed to initialize I18N. Language: {lang}, with key: {key} has no value")]
    MissingValue { lang: String, key: String },
}

pub fn load() -> Result<HashMap<Language, FluentLanguageLoader>, I18nLoaderError> {
    let loaders: HashMap<Language, FluentLanguageLoader> = Language::iter()
        .map(|language| {
            let loader = FluentLanguageLoader::new(DOMAIN, language.lang_id());
            i18n_embed::select(&loader, &Localizations, &[language.lang_id()])?;
            Ok((language, loader))
        })
        .collect::<Result<HashMap<Language, FluentLanguageLoader>, I18nLoaderError>>()?;

    // Validatie logica (was make_sure_i18n_is_properly_initialized)
    for language in Language::iter() {
        let loader = loaders.get(&language).expect("Loader moet bestaan"); // Kan niet misgaan hierboven

        for key in TextKeys::iter() {
            let key_str = key.to_string();

            if !loader.has(&key_str) {
                let err = I18nLoaderError::MissingKey {
                    lang: language.to_string(),
                    key: key_str,
                };
                // Direct printen naar stderr zoals gevraagd
                eprintln!("Build Error: {}", err);
                return Err(err);
            }

            let value = loader.get(&key_str);
            if value.trim().is_empty() {
                let err = I18nLoaderError::MissingValue {
                    lang: language.to_string(),
                    key: key_str,
                };
                eprintln!("Build Error: {}", err);
                return Err(err);
            }
        }
    }

    Ok(loaders)
}