use crate::domain::errors::ServerError;
use crate::domain::tokens::Tokens;
use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;

#[derive(Debug)]
pub struct FakeTokenStoreFileAdapter {
    pub tokens: Option<Tokens>
}

pub const TEST_FILE_ACCESS_TOKEN: &str = "test file access token";
pub const TEST_FILE_REFRESH_TOKEN: &str = "test file refresh token";
pub const TEST_FILE_EXPIRES_AT: i64 = i64::MAX -1;


impl FakeTokenStoreFileAdapter {
    pub fn with_tokens() -> Self {
        FakeTokenStoreFileAdapter {
            tokens: Some(Tokens {
                access_token: TEST_FILE_ACCESS_TOKEN.to_string(),
                refresh_token: TEST_FILE_REFRESH_TOKEN.to_string(),
                expires_at: TEST_FILE_EXPIRES_AT,
            }),
        }
    }
    pub fn empty() -> Self {
        FakeTokenStoreFileAdapter { tokens: None }
    }
}

impl TokenStoreDrivenPort for FakeTokenStoreFileAdapter {
    fn is_available(&self) -> bool {
        true
    }

    fn load(&self) -> Result<Option<Tokens>, ServerError> {
        Ok(self.tokens.clone())
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ServerError> {
        _ = tokens;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FakeTokenStoreRingAdapter {
    pub tokens: Option<Tokens>
}

pub const TEST_RING_ACCESS_TOKEN: &str = "test ring access token";
pub const TEST_RING_REFRESH_TOKEN: &str = "test ring refresh token";
pub const TEST_RING_EXPIRES_AT: i64 = i64::MAX;

impl FakeTokenStoreRingAdapter {
    pub fn with_tokens() -> Self {
        FakeTokenStoreRingAdapter {
            tokens: Some(Tokens {
                access_token: TEST_RING_ACCESS_TOKEN.to_string(),
                refresh_token: TEST_RING_REFRESH_TOKEN.to_string(),
                expires_at: TEST_RING_EXPIRES_AT,
            }),
        }
    }
    pub fn empty() -> Self {
        FakeTokenStoreRingAdapter { tokens: None }
    }
}
impl TokenStoreDrivenPort for FakeTokenStoreRingAdapter {
    fn is_available(&self) -> bool {
        true
    }

    fn load(&self) -> Result<Option<Tokens>, ServerError> {
        Ok(self.tokens.clone())
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ServerError> {
        _ = tokens;
        Ok(())
    }
}

