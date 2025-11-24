use engine::domain::default_values::general_defaults::*;
use engine::domain::errors::ConfigurationError;
use engine::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
use serde::{Deserialize, Serialize};
use keyring::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyRingTokenStoreAdapter {
    access_token: String,
    refresh_token: String,
    expires_at: i64,
}

impl TokenStoreDrivenPort for KeyRingTokenStoreAdapter {
    fn load() -> Result<Self, ConfigurationError>
    where
        Self: Sized
    {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
            .map_err(|e| ConfigurationError::CouldNotAccessKeyring(e.to_string()))?;

        let json = entry
            .get_password()
            .map_err(|e| ConfigurationError::CouldNotReadTokensFromKeyring(e.to_string()))?;

        let tokens: KeyRingTokenStoreAdapter = serde_json::from_str(&json)
            .map_err(|e| ConfigurationError::CouldNotParseJson(e.to_string()))?;

        Ok(tokens)
    }

    fn save(&self) -> Result<(), ConfigurationError> {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
            .map_err(|e| ConfigurationError::CouldNotAccessKeyring(e.to_string()))?;

        let json = serde_json::to_string(self)
            .map_err(|e| ConfigurationError::CouldNotSerializeTokens(e.to_string()))?;

        entry
            .set_password(&json)
            .map_err(|e| ConfigurationError::CouldNotSaveTokensToKeyring(e.to_string()))?;

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