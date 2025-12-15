use strum_macros::{Display, EnumIter, VariantNames};
use unic_langid::LanguageIdentifier;

#[derive(EnumIter, Display, VariantNames, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Language {
    #[strum(serialize = "en-GB")]
    EnGb,
    #[strum(serialize = "nl-NL")]
    NlNl
}

impl Language {
    pub fn lang_id(&self) -> LanguageIdentifier {
        self.to_string()
            .parse()
            .expect("Invalid language identifier")
    }
}