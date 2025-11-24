use crate::domain::errors::ConfigurationError;
use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

#[derive(Debug)]
pub struct FakeTokenStoreFileAdapter {
    access_token: String,
    refresh_token: String,
    expires_at: i64,
}

pub const TEST_FILE_ACCESS_TOKEN: &str = "test file access token";
pub const TEST_FILE_REFRESH_TOKEN: &str = "test file refresh token";
pub const TEST_FILE_EXPIRES_AT: i64 = i64::MAX -1;

impl TokenStoreDrivenPort for FakeTokenStoreFileAdapter {
    fn load() -> Result<Self, ConfigurationError>
    {
        Ok( FakeTokenStoreFileAdapter {
                access_token: TEST_FILE_ACCESS_TOKEN.to_string(),
                refresh_token: TEST_FILE_REFRESH_TOKEN.to_string(),
                expires_at: TEST_FILE_EXPIRES_AT
        })
    }

    fn save(&self) -> Result<(), ConfigurationError> {
       Ok(())
    }

    fn access_token(&self) -> String {
        self.access_token.clone()
    }

    fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }

    fn expires_at(&self) -> i64 {
        self.expires_at
    }
}

#[derive(Debug)]
pub struct FakeTokenStoreRingAdapter {
    access_token: String,
    refresh_token: String,
    expires_at: i64,
}

pub const TEST_RING_ACCESS_TOKEN: &str = "test ring access token";
pub const TEST_RING_REFRESH_TOKEN: &str = "test ring refresh token";
pub const TEST_RING_EXPIRES_AT: i64 = i64::MAX;

impl TokenStoreDrivenPort for FakeTokenStoreRingAdapter {
    fn load() -> Result<Self, ConfigurationError>
    {
        Ok( FakeTokenStoreRingAdapter {
            access_token: TEST_RING_ACCESS_TOKEN.to_string(),
            refresh_token: TEST_RING_REFRESH_TOKEN.to_string(),
            expires_at: TEST_RING_EXPIRES_AT
        })
    }

    fn save(&self) -> Result<(), ConfigurationError> {
        Ok(())
    }

    fn access_token(&self) -> String {
        self.access_token.clone()
    }

    fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }

    fn expires_at(&self) -> i64 {
        self.expires_at
    }
}

