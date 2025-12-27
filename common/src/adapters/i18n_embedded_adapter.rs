use std::collections::HashMap;
use std::sync::Arc;
use i18n_embed::fluent::FluentLanguageLoader;
use crate::domain::defaults::*;
use crate::ports::i18n_driven_port::I18nDrivenPort;
use fluent_bundle::{FluentArgs, FluentValue};
use i18n_loader::{Language, TextKeys, load as load_i18n};

#[derive(Clone)]
pub struct I18nEmbeddedFtlAdapter {
    selected_language: Language,
    loaders: Arc<HashMap<Language, FluentLanguageLoader>>
}

impl I18nEmbeddedFtlAdapter {
    pub fn load() -> I18nEmbeddedFtlAdapter {

        let loaders = load_i18n()
            .expect("This error should have been handled during build time in the build.rs");

       I18nEmbeddedFtlAdapter { selected_language: DEFAULT_LANGUAGE, loaders: Arc::new(loaders) }
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
    use i18n_loader::{Language, TextKeys};
    use crate::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;
    use crate::domain::defaults::DEFAULT_LANGUAGE;
    use crate::ports::i18n_driven_port::I18nDrivenPort;

    #[test]
    fn should_be_able_to_load_localization_files() {
        // Calling load should not trigger the expect call
        I18nEmbeddedFtlAdapter::load();
        assert!(true);
    }

    #[test]
    fn t_uses_default_language() {
        let service = I18nEmbeddedFtlAdapter::load();

        let default_value = service.t(TextKeys::AuthenticateBtn);
        let explicit_value =
            service.t_by_lang(DEFAULT_LANGUAGE, TextKeys::AuthenticateBtn);

        assert_eq!(default_value, explicit_value);
    }

    #[test]
    fn adding_a_new_text_key_requires_translations() {
        let service = I18nEmbeddedFtlAdapter::load();

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