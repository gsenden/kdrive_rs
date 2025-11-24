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

    fn access_token(&self) -> String {
        match (&self.key_ring_store, &self.file_store) {
            (Some(ring), _) => ring.access_token(),
            (None, Some(file)) => file.access_token(),
            (None, None) => unreachable!(
                "Both ports are None; constructor should have prevented this."
            ),
        }
    }
    fn refresh_token(&self) -> String {
        match (&self.key_ring_store, &self.file_store) {
            (Some(ring), _) => ring.refresh_token(),
            (None, Some(file)) => file.refresh_token(),
            (None, None) => unreachable!(
                "Both ports are None; constructor should have prevented this."
            ),
        }
    }

    fn expires_at(&self) -> i64 {
        match (&self.key_ring_store, &self.file_store) {
            (Some(ring), _) => ring.expires_at(),
            (None, Some(file)) => file.expires_at(),
            (None, None) => unreachable!(
                "Both ports are None; constructor should have prevented this."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::errors::ConfigurationError;
    use crate::domain::test_helpers::fake_token_store_adapter::{FakeTokenStoreFileAdapter, FakeTokenStoreRingAdapter, TEST_FILE_ACCESS_TOKEN, TEST_FILE_EXPIRES_AT, TEST_FILE_REFRESH_TOKEN, TEST_RING_ACCESS_TOKEN, TEST_RING_EXPIRES_AT, TEST_RING_REFRESH_TOKEN};
    use crate::domain::token_store::TokenStore;
    use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

    #[test]
    fn when_creating_a_new_store_at_least_one_port_should_be_some() {
        // Given a fake token store adapter
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        // When creating a new token store
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(None, Some(file_store_adapter)).unwrap();

        // Then no error should be raised
        assert!(store.file_store.is_some());
    }

    #[test]
    fn when_creating_a_new_store_with_both_ports_none_should_return_an_error() {
        // When creating a new token store
        let store_result =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(None, None);

        // Then the correct error should be returned
        assert_eq!(store_result.unwrap_err(), ConfigurationError::MissingStorePort);
    }

    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_access_token_should_be_returned() {
        // Given a fake file store adapter
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        // And a token store with only the file store adapter
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(None, Some(file_store_adapter)).unwrap();

        // When getting the access token
        let access_token = store.access_token();

        // Then the correct access token should be returned
        assert_eq!(access_token, TEST_FILE_ACCESS_TOKEN);
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_access_token_should_be_returned() {
        // Given a fake file store adapter
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();

        // And a token store with only the file store adapter
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(Some(ring_store_adapter), None).unwrap();

        // When getting the access token
        let access_token = store.access_token();

        // Then the correct access token should be returned
        assert_eq!(access_token, TEST_RING_ACCESS_TOKEN);
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_access_token_should_be_returned() {
        // Given the fake store adapters
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        // And a token store with both store adapters
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(Some(ring_store_adapter), Some(file_store_adapter)).unwrap();

        // When getting the access token
        let access_token = store.access_token();

        // Then the correct access token should be returned
        assert_eq!(access_token, TEST_RING_ACCESS_TOKEN);
    }

    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_refresh_token_should_be_returned() {
        // Given a fake file store adapter
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        // And a token store with only the file store adapter
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(None, Some(file_store_adapter)).unwrap();

        // When getting the access token
        let refresh_token = store.refresh_token();

        // Then the correct access token should be returned
        assert_eq!(refresh_token, TEST_FILE_REFRESH_TOKEN);
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_refresh_token_should_be_returned() {
        // Given a fake file store adapter
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();

        // And a token store with only the file store adapter
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(Some(ring_store_adapter), None).unwrap();

        // When getting the access token
        let refresh_token = store.refresh_token();

        // Then the correct access token should be returned
        assert_eq!(refresh_token, TEST_RING_REFRESH_TOKEN);
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_refresh_token_should_be_returned() {
        // Given the fake store adapters
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        // And a token store with both store adapters
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(Some(ring_store_adapter), Some(file_store_adapter)).unwrap();

        // When getting the access token
        let refresh_token = store.refresh_token();

        // Then the correct access token should be returned
        assert_eq!(refresh_token, TEST_RING_REFRESH_TOKEN);
    }

    #[test]
    fn when_the_store_has_only_a_file_port_the_correct_expires_at_should_be_returned() {
        // Given a fake file store adapter
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        // And a token store with only the file store adapter
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(None, Some(file_store_adapter)).unwrap();

        // When getting the access token
        let expires_at = store.expires_at();

        // Then the correct access token should be returned
        assert_eq!(expires_at, TEST_FILE_EXPIRES_AT);
    }

    #[test]
    fn when_the_store_has_only_a_ring_port_the_correct_expires_at_should_be_returned() {
        // Given a fake file store adapter
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();

        // And a token store with only the file store adapter
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(Some(ring_store_adapter), None).unwrap();

        // When getting the access token
        let expires_at = store.expires_at();

        // Then the correct access token should be returned
        assert_eq!(expires_at, TEST_RING_EXPIRES_AT);
    }

    #[test]
    fn when_the_store_has_both_ports_the_correct_expires_at_should_be_returned() {
        // Given the fake store adapters
        let ring_store_adapter = FakeTokenStoreRingAdapter::load().unwrap();
        let file_store_adapter = FakeTokenStoreFileAdapter::load().unwrap();

        // And a token store with both store adapters
        let store =
            TokenStore::<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>
            ::new(Some(ring_store_adapter), Some(file_store_adapter)).unwrap();

        // When getting the access token
        let expires_at = store.expires_at();

        // Then the correct access token should be returned
        assert_eq!(expires_at, TEST_RING_EXPIRES_AT);
    }
}