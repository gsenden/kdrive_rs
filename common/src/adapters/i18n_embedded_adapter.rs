use std::collections::HashMap;
use std::sync::Arc;
use i18n_embed::fluent::FluentLanguageLoader;
use rust_embed::RustEmbed;
use strum::IntoEnumIterator;
use crate::domain::defaults::*;
use crate::domain::errors::CommonError;
use crate::domain::language::Language;
use crate::domain::text_keys::TextKeys;
use crate::ports::i18n_driven_port::I18nDrivenPort;
use fluent_bundle::{FluentArgs, FluentValue};

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

#[derive(Clone)]
pub struct I18nEmbeddedFtlAdapter {
    selected_language: Language,
    loaders: Arc<HashMap<Language, FluentLanguageLoader>>
}

impl I18nEmbeddedFtlAdapter {
    pub fn load() -> Result<I18nEmbeddedFtlAdapter, CommonError>{

        let loaders: HashMap<Language, FluentLanguageLoader> = Language::iter()
            .map(|language| {
                let loader = FluentLanguageLoader::new(DOMAIN, language.lang_id());
                i18n_embed::select(&loader,&Localizations,&[language.lang_id()])?;
                Ok((language,loader))
            })
            .collect::<Result<HashMap<Language, FluentLanguageLoader>, CommonError>>()?;

        let service = I18nEmbeddedFtlAdapter { selected_language: DEFAULT_LANGUAGE, loaders: Arc::new(loaders) };

        service.make_sure_i18n_is_properly_initialized()?;

        Ok(service)
    }
    fn make_sure_i18n_is_properly_initialized(&self) -> Result<(), CommonError> {
        for language in Language::iter() {
            let loader = self
                .loaders
                .get(&language)
                .ok_or_else(|| { CommonError::I18NMissingLanguage(language.to_string())})?;

            for key in TextKeys::iter() {
                let key_str = key.to_string();

                if !loader.has(&key_str) {
                    return Err(CommonError::I18NMissingKeyError(language.to_string(),key_str));
                }

                let value = self.t_by_lang(language,key);
                if value.trim().is_empty() {
                    return Err(CommonError::I18NMissingValueError(language.to_string(),key_str));
                }
            }
        }
        Ok(())
    }
}

impl PartialEq for I18nEmbeddedFtlAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.selected_language == other.selected_language
            && Arc::ptr_eq(&self.loaders, &other.loaders)
    }
}


impl I18nDrivenPort for I18nEmbeddedFtlAdapter {
    fn t(&self, key: TextKeys) -> String {
        self.t_by_lang(self.selected_language, key)
    }
    fn t_by_lang(&self, language: Language, key: TextKeys) -> String {
        self.loaders[&language].get(&key.to_string())
    }
    fn t_with_args( &self, key: TextKeys, args: &[(&'static str, String)] ) -> String {
        let mut fluent_args = FluentArgs::new();

        for (name, value) in args {
            fluent_args.set(*name, FluentValue::from(value.clone()));
        }

        let args_map: HashMap<String, FluentValue> =
            fluent_args
                .into_iter()
                .map(|(k, v)| (k.into_owned(), v))
                .collect();

        self.loaders[&self.selected_language]
            .get_args(&key.to_string(), args_map)
    }
}


#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;
    use crate::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
    use crate::domain::defaults::DEFAULT_LANGUAGE;
    use crate::domain::language::Language;
    use crate::domain::text_keys::TextKeys;
    use crate::ports::i18n_driven_port::I18nDrivenPort;

    #[test]
    fn should_be_able_to_load_localization_files() {
        let service = I18nEmbeddedFtlAdapter::load();
        assert!(service.is_ok());
    }

    #[test]
    fn load_succeeds_when_i18n_is_valid() {
        assert!(I18nEmbeddedFtlAdapter::load().is_ok());
    }

    #[test]
    fn all_keys_exist_for_all_languages() {
        let service = I18nEmbeddedFtlAdapter::load().unwrap();
        let result = service.make_sure_i18n_is_properly_initialized();
        assert!(result.is_ok());
    }


    #[test]
    fn t_uses_default_language() {
        let service = I18nEmbeddedFtlAdapter::load().unwrap();

        let default_value = service.t(TextKeys::AuthenticateBtn);
        let explicit_value =
            service.t_by_lang(DEFAULT_LANGUAGE, TextKeys::AuthenticateBtn);

        assert_eq!(default_value, explicit_value);
    }

    #[test]
    fn adding_a_new_text_key_requires_translations() {
        let service = I18nEmbeddedFtlAdapter::load().unwrap();

        let key_count = TextKeys::iter().count();
        assert!(key_count > 0);

        for language in Language::iter() {
            let values = TextKeys::iter()
                .map(|k| service.t_by_lang(language, k))
                .collect::<Vec<_>>();

            assert!(
                values.iter().all(|v| !v.trim().is_empty()),
                "One or more translations missing for {:?}",
                language
            );
        }
    }

}