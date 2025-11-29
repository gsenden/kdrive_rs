use crate::domain::errors::ConfigurationError;
use crate::domain::tokens::Tokens;
use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

#[derive(Debug)]
pub struct FakeTokenStoreFileAdapter;

pub const TEST_FILE_ACCESS_TOKEN: &str = "test file access token";
pub const TEST_FILE_REFRESH_TOKEN: &str = "test file refresh token";
pub const TEST_FILE_EXPIRES_AT: i64 = i64::MAX -1;

impl TokenStoreDrivenPort for FakeTokenStoreFileAdapter {
    fn is_available(&self) -> bool {
        true
    }

    fn load(&self) -> Result<Option<Tokens>, ConfigurationError> {
        let tokens = Tokens {
            access_token: TEST_FILE_ACCESS_TOKEN.to_string(),
            refresh_token: TEST_FILE_REFRESH_TOKEN.to_string(),
            expires_at: TEST_FILE_EXPIRES_AT
        };
        Ok(Some(tokens))
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ConfigurationError> {
        _ = tokens;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FakeTokenStoreRingAdapter;

pub const TEST_RING_ACCESS_TOKEN: &str = "test ring access token";
pub const TEST_RING_REFRESH_TOKEN: &str = "test ring refresh token";
pub const TEST_RING_EXPIRES_AT: i64 = i64::MAX;

impl TokenStoreDrivenPort for FakeTokenStoreRingAdapter {
    fn is_available(&self) -> bool {
        true
    }

    fn load(&self) -> Result<Option<Tokens>, ConfigurationError> {
        let tokens = Tokens {
            access_token: TEST_RING_ACCESS_TOKEN.to_string(),
            refresh_token: TEST_RING_REFRESH_TOKEN.to_string(),
            expires_at: TEST_RING_EXPIRES_AT
        };
        Ok(Some(tokens))
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ConfigurationError> {
        _ = tokens;
        Ok(())
    }
}

