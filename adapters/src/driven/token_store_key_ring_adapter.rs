use engine::domain::default_values::general_defaults::*;
use engine::domain::errors::ConfigurationError;
use engine::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
use serde::{Deserialize, Serialize};
use keyring::Entry;
use engine::domain::tokens::Tokens;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStoreKeyRingAdapter;

impl TokenStoreDrivenPort for TokenStoreKeyRingAdapter {
    fn is_available(&self) -> bool {
        Entry::new(KEYRING_SERVICE, KEYRING_USER).is_ok()
    }

    fn load(&self) -> Result<Option<Tokens>, ConfigurationError> {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
            .map_err(|e| ConfigurationError::CouldNotAccessKeyring(e.to_string()))?;

        let json = match entry.get_password() {
            Ok(json) => json,
            Err(error) => {
                if error.to_string().contains("No matching entry") {
                    return Ok(None);
                }
                return Err(ConfigurationError::CouldNotReadTokensFromKeyring(error.to_string()));
            }
        };

        let tokens: Tokens = serde_json::from_str(&json).map_err(|e| {
            ConfigurationError::CouldNotParseJson(e.to_string())
        })?;

        Ok(Some(tokens))
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ConfigurationError> {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
            .map_err(|e| ConfigurationError::CouldNotAccessKeyring(e.to_string()))?;

        let json = serde_json::to_string(tokens).map_err(|e| {
            ConfigurationError::CouldNotSerializeTokens(e.to_string())
        })?;

        entry
            .set_password(&json)
            .map_err(|e| ConfigurationError::CouldNotSaveTokensToKeyring(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_store_key_ring_adapter_returns_ok_none_when_entry_not_found() {
        // Given a KeyRingAdapter
        let adapter = TokenStoreKeyRingAdapter;

        // When we load tokens (entry doesn't exist)
        let result = adapter.load();

        // Then we get Ok(None), not an error
        assert_eq!(result.unwrap(), None);
    }
}
