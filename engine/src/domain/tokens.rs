use serde::{Deserialize, Serialize};
use crate::domain::errors::ConfigurationError;
use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ActivePort<TRP, TFP> {
    KeyRing(TRP),
    File(TFP)
}

impl<TRP: TokenStoreDrivenPort, TFP: TokenStoreDrivenPort> TokenStoreDrivenPort for ActivePort<TRP, TFP> {
    fn is_available(&self) -> bool {
        match self {
            ActivePort::KeyRing(p) => p.is_available(),
            ActivePort::File(p) => p.is_available(),
        }
    }

    fn load(&self) -> Result<Option<Tokens>, ConfigurationError> {
        match self {
            ActivePort::KeyRing(p) => p.load(),
            ActivePort::File(p) => p.load(),
        }
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ConfigurationError> {
        match self {
            ActivePort::KeyRing(p) => p.save(tokens),
            ActivePort::File(p) => p.save(tokens),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenStore<TRP, TFP>
where
    TRP: TokenStoreDrivenPort,
    TFP: TokenStoreDrivenPort
{
    tokens: Tokens,
    port: ActivePort<TRP, TFP>
}

impl <TRP: TokenStoreDrivenPort, TFP: TokenStoreDrivenPort>TokenStore<TRP, TFP> {
    pub fn new(
        tokens: Tokens,
        key_ring_store: Option<TRP>,
        file_store: Option<TFP>
    ) -> Result<Self, ConfigurationError> {
        let port = TokenStore::choose_port(key_ring_store, file_store)?;
        Ok(TokenStore {port, tokens})
    }

    pub fn load(key_ring_store: Option<TRP>,
                file_store: Option<TFP>) -> Result<Self, ConfigurationError> {
        let port = TokenStore::choose_port(key_ring_store, file_store)?;
        let tokens = port.load()?.ok_or_else(|| ConfigurationError::NoTokensFoundInStore)?;
        Ok(TokenStore {tokens, port})
    }

    fn choose_port(key_ring_store: Option<TRP>,   file_store: Option<TFP>)
                   -> Result<ActivePort<TRP, TFP>, ConfigurationError>
    {
        if let Some(key_ring_store) = key_ring_store {
            if key_ring_store.is_available() {
                return Ok(ActivePort::KeyRing(key_ring_store))
            }
        }

        if let Some(file_store) = file_store {
            if file_store.is_available() {
                return Ok(ActivePort::File(file_store))
            }
        }

        Err(ConfigurationError::MissingStorePort)
    }
}

impl <TRP: TokenStoreDrivenPort, TFP: TokenStoreDrivenPort>TokenStore<TRP, TFP> {
    fn access_token(&self) -> &str {
        &self.tokens.access_token
    }

    fn refresh_token(&self) -> &str {
        &self.tokens.refresh_token
    }

    fn expires_at(&self) -> i64 {
        self.tokens.expires_at
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::errors::ConfigurationError;
    use crate::domain::test_helpers::fake_token_store_adapter::*;
    use crate::domain::tokens::TokenStore;
    use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

    type TestStore = TokenStore<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>;

    fn file_only_store() -> TestStore {
        let file_store_adapter = FakeTokenStoreFileAdapter;
        TestStore::load(None, Some(file_store_adapter)).unwrap()
    }

    fn ring_only_store() -> TestStore {
        let ring_store_adapter = FakeTokenStoreRingAdapter;
        TestStore::load(Some(ring_store_adapter), None).unwrap()
    }

    fn both_store() -> TestStore {
        let ring_store_adapter = FakeTokenStoreRingAdapter;
        let file_store_adapter = FakeTokenStoreFileAdapter;
        TestStore::load(Some(ring_store_adapter), Some(file_store_adapter)).unwrap()
    }

    #[test]
    fn when_creating_a_new_store_at_least_one_port_should_be_some() {
        let file_store_adapter = FakeTokenStoreFileAdapter;
        let tokens = file_store_adapter.load().unwrap().unwrap();

        let store_result = TestStore::new(tokens,None, Some(file_store_adapter));

        assert!(store_result.is_ok());
    }

    #[test]
    fn when_creating_a_new_store_with_both_ports_none_should_return_an_error() {
        let file_store_adapter = FakeTokenStoreFileAdapter;
        let tokens = file_store_adapter.load().unwrap().unwrap();

        let store_result = TestStore::new(tokens, None, None);

        assert_eq!(
            store_result.unwrap_err(),
            ConfigurationError::MissingStorePort
        );
    }

    // access_token
    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_access_token_should_be_returned() {
        let store = file_only_store();
        assert_eq!(store.access_token(), TEST_FILE_ACCESS_TOKEN);
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_access_token_should_be_returned() {
        let store = ring_only_store();
        assert_eq!(store.access_token(), TEST_RING_ACCESS_TOKEN);
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_access_token_should_be_returned() {
        let store = both_store();
        assert_eq!(store.access_token(), TEST_RING_ACCESS_TOKEN);
    }

    // refresh_token
    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_refresh_token_should_be_returned() {
        let store = file_only_store();
        assert_eq!(store.refresh_token(), TEST_FILE_REFRESH_TOKEN);
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_refresh_token_should_be_returned() {
        let store = ring_only_store();
        assert_eq!(store.refresh_token(), TEST_RING_REFRESH_TOKEN);
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_refresh_token_should_be_returned() {
        let store = both_store();
        assert_eq!(store.refresh_token(), TEST_RING_REFRESH_TOKEN);
    }

    // expires_at

    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_expires_at_should_be_returned() {
        let store = file_only_store();
        assert_eq!(store.expires_at(), TEST_FILE_EXPIRES_AT);
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_expires_at_should_be_returned() {
        let store = ring_only_store();
        assert_eq!(store.expires_at(), TEST_RING_EXPIRES_AT);
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_expires_at_should_be_returned() {
        let store = both_store();
        assert_eq!(store.expires_at(), TEST_RING_EXPIRES_AT);
    }
}