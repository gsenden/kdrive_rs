use common::ports::i18n_driven_port::I18nDrivenPort;
use common::domain::language::Language;
use common::domain::text_keys::TextKeys;

#[derive(Clone, Debug)]
pub struct FakeI18n;

impl PartialEq for FakeI18n {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl I18nDrivenPort for FakeI18n {

    fn t(&self, key: TextKeys) -> String {
        // deterministic, makkelijk te testen
        format!("[{:?}]", key)
    }

    fn t_by_lang(&self, _language: Language, key: TextKeys) -> String {
        self.t(key)
    }

    fn t_with_args(
        &self,
        key: TextKeys,
        args: &[(&'static str, String)],
    ) -> String {
        if args.is_empty() {
            format!("[{:?}]", key)
        } else {
            let args_str = args
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(", ");

            format!("[{:?} | {}]", key, args_str)
        }
    }
}