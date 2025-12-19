use crate::domain::language::Language;
use crate::domain::text_keys::TextKeys;

pub trait I18nDrivenPort: Clone + Send + Sync + PartialEq {
    fn t(&self, key: TextKeys) -> String;
    fn t_by_lang(&self, language: Language, key: TextKeys) -> String;
    fn t_with_args(&self, key: TextKeys, args: &[(/* param name */ &'static str, String)]) -> String;
}