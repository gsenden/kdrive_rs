use serde::{Deserialize, Serialize};
use crate::domain::errors::ConfigurationError;
use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

#[derive(Debug, Serialize, Deserialize)]
#[derive(PartialEq)]
pub struct TokenStore<TRP, TFP>
where
    TRP: TokenStoreDrivenPort,
    TFP: TokenStoreDrivenPort
{
    key_ring_store: Option<TRP>,
    file_store: Option<TFP>,
}

impl <TRP: TokenStoreDrivenPort, TFP: TokenStoreDrivenPort>TokenStore<TRP, TFP> {
    pub fn new(key_ring_store: Option<TRP>, file_store: Option<TFP>) -> Result<Self, ConfigurationError> {
        if key_ring_store.is_none() && file_store.is_none() {
            Err(ConfigurationError::MissingStorePort)
        } else {
            Ok(Self { key_ring_store, file_store })
        }
    }
}

impl <TRP: TokenStoreDrivenPort, TFP: TokenStoreDrivenPort>TokenStore<TRP, TFP> {

    fn with_store<R>(
        &self,
        f_ring: impl FnOnce(&TRP) -> R,
        f_file: impl FnOnce(&TFP) -> R,
    ) -> R {
        match (&self.key_ring_store, &self.file_store) {
            (Some(ring), _) => f_ring(ring),
            (None, Some(file)) => f_file(file),
            (None, None) => unreachable!(
                "Both ports are None; constructor should have prevented this."
            ),
        }
    }

    fn access_token(&self) -> String {
        self.with_store(
            |ring| ring.access_token(),
            |file| file.access_token(),
        )
    }

    fn refresh_token(&self) -> String {
        self.with_store(
            |ring| ring.refresh_token(),
            |file| file.refresh_token(),
        )
    }

    fn expires_at(&self) -> i64 {
        self.with_store(
            |ring| ring.expires_at(),
            |file| file.expires_at(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::errors::ConfigurationError;
    use crate::domain::test_helpers::fake_token_store_adapter::*;
    use crate::domain::token_store::TokenStore;
    use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

    type TestStore = TokenStore<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>;

    fn file_only_store() -> TestStore {
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();
        TestStore::new(None, Some(file_store_adapter)).unwrap()
    }

    fn ring_only_store() -> TestStore {
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();
        TestStore::new(Some(ring_store_adapter), None).unwrap()
    }

    fn both_store() -> TestStore {
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();
        TestStore::new(Some(ring_store_adapter), Some(file_store_adapter)).unwrap()
    }

    #[test]
    fn when_creating_a_new_store_at_least_one_port_should_be_some() {
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        let store_result = TestStore::new(None, Some(file_store_adapter));

        assert!(store_result.is_ok());
    }

    #[test]
    fn when_creating_a_new_store_with_both_ports_none_should_return_an_error() {
        let store_result = TestStore::new(None, None);

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