use serde::{Deserialize, Serialize};
use crate::domain::errors::ServerError;
use crate::error;
use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
use crate::ports::driving::token_store_driving_port::TokenStoreDrivingPort;

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

    fn load(&self) -> Result<Option<Tokens>, ServerError> {
        match self {
            ActivePort::KeyRing(p) => p.load(),
            ActivePort::File(p) => p.load(),
        }
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ServerError> {
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
    tokens: Option<Tokens>,
    port: ActivePort<TRP, TFP>
}

impl <TRP: TokenStoreDrivenPort, TFP: TokenStoreDrivenPort>TokenStore<TRP, TFP> {
    pub fn new(
        tokens: Tokens,
        key_ring_store: Option<TRP>,
        file_store: Option<TFP>
    ) -> Result<Self, ServerError> {
        let port = TokenStore::choose_port(key_ring_store, file_store)?;
        Ok(TokenStore {tokens: Some(tokens), port})
    }

    pub fn load(key_ring_store: Option<TRP>,
                file_store: Option<TFP>) -> Result<Self, ServerError> {
        let port = TokenStore::choose_port(key_ring_store, file_store)?;
        let tokens = port.load()?;
        Ok(TokenStore {tokens, port})
    }

    fn choose_port(key_ring_store: Option<TRP>,   file_store: Option<TFP>)
                   -> Result<ActivePort<TRP, TFP>, ServerError>
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
        Err(error!(MissingStorePort))
    }
}

// engine/src/domain/token_store.rs
impl<TRP: TokenStoreDrivenPort, TFP: TokenStoreDrivenPort> TokenStoreDrivingPort
for TokenStore<TRP, TFP>
{
    fn has_tokens(&self) -> bool {
        self.tokens.is_some()
    }

    fn access_token(&self) -> Option<&str> {
        match &self.tokens {
            Some(t) => Some(t.access_token.as_str()),
            None => None,
        }
    }

    fn refresh_token(&self) -> Option<&str> {
        match &self.tokens {
            Some(t) => Some(t.refresh_token.as_str()),
            None => None,
        }
    }

    fn expires_at(&self) -> Option<i64> {
        match &self.tokens {
            Some(t) => Some(t.expires_at),
            None => None,
        }
    }

    fn save_tokens(&mut self, tokens: &Tokens) -> Result<(), ServerError> {
        self.port.save(tokens)?;
        self.tokens = Some(tokens.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::errors::ServerError;
    use crate::domain::test_helpers::fake_token_store_adapter::*;
    use crate::domain::test_helpers::test_store::TestStore;
    use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
    use crate::ports::driving::token_store_driving_port::TokenStoreDrivingPort;

    fn file_only_store() -> TestStore {
        let file_store_adapter = FakeTokenStoreFileAdapter::with_tokens();
        TestStore::load(None, Some(file_store_adapter)).unwrap()
    }

    fn ring_only_store() -> TestStore {
        let ring_store_adapter = FakeTokenStoreRingAdapter::with_tokens();
        TestStore::load(Some(ring_store_adapter), None).unwrap()
    }

    fn both_store() -> TestStore {
        let ring_store_adapter = FakeTokenStoreRingAdapter::with_tokens();
        let file_store_adapter = FakeTokenStoreFileAdapter::with_tokens();
        TestStore::load(Some(ring_store_adapter), Some(file_store_adapter)).unwrap()
    }

    #[test]
    fn when_creating_a_new_store_at_least_one_port_should_be_some() {
        let file_store_adapter = FakeTokenStoreFileAdapter::with_tokens();
        let tokens = file_store_adapter.load().unwrap().unwrap();

        let store_result = TestStore::new(tokens,None, Some(file_store_adapter));

        assert!(store_result.is_ok());
    }

    #[test]
    fn when_creating_a_new_store_with_both_ports_none_should_return_an_error() {
        let file_store_adapter = FakeTokenStoreFileAdapter::with_tokens();
        let tokens = file_store_adapter.load().unwrap().unwrap();

        let store_result = TestStore::new(tokens, None, None);

        assert!(matches!(
        store_result,
        Err(ServerError::Localized(common::domain::errors::LocalizedError {
            key: common::domain::text_keys::TextKeys::MissingStorePort,
            ..
        }))
    ));
    }

    // access_token
    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_access_token_should_be_returned() {
        let store = file_only_store();
        assert_eq!(store.access_token(), Some(TEST_FILE_ACCESS_TOKEN));
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_access_token_should_be_returned() {
        let store = ring_only_store();
        assert_eq!(store.access_token(), Some(TEST_RING_ACCESS_TOKEN));
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_access_token_should_be_returned() {
        let store = both_store();
        assert_eq!(store.access_token(), Some(TEST_RING_ACCESS_TOKEN));
    }

    // refresh_token
    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_refresh_token_should_be_returned() {
        let store = file_only_store();
        assert_eq!(store.refresh_token(), Some(TEST_FILE_REFRESH_TOKEN));
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_refresh_token_should_be_returned() {
        let store = ring_only_store();
        assert_eq!(store.refresh_token(), Some(TEST_RING_REFRESH_TOKEN));
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_refresh_token_should_be_returned() {
        let store = both_store();
        assert_eq!(store.refresh_token(), Some(TEST_RING_REFRESH_TOKEN));
    }

    // expires_at

    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_expires_at_should_be_returned() {
        let store = file_only_store();
        assert_eq!(store.expires_at(), Some(TEST_FILE_EXPIRES_AT));
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_expires_at_should_be_returned() {
        let store = ring_only_store();
        assert_eq!(store.expires_at(), Some(TEST_RING_EXPIRES_AT));
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_expires_at_should_be_returned() {
        let store = both_store();
        assert_eq!(store.expires_at(), Some(TEST_RING_EXPIRES_AT));
    }

    #[test]
    fn token_store_without_tokens_returns_that_it_has_no_tokens() {
        // Given a token store without tokens
        let ring_adapter = FakeTokenStoreRingAdapter::empty();
        let store =
            TestStore::load(Some(ring_adapter), None).unwrap();

        // When has_tokens is called
        let result = store.has_tokens();

        // Then it returns false
        assert_eq!(result, false);
    }
}