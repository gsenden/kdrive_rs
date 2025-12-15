use crate::domain::errors::CommonError;
use crate::domain::language::Language;
use crate::domain::text_keys::TextKeys;

pub trait I18nDrivenPort: Sized {
    fn load() -> Result<Self, CommonError>;
    fn t(&self, key: TextKeys) -> String;
    fn t_by_lang(&self, language: Language, key: TextKeys) -> String;
}