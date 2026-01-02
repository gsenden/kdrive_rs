use engine::domain::default_values::general_defaults::*;
use engine::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
use serde::{Deserialize, Serialize};
use keyring::Entry;
use common::application_error;
use common::domain::errors::ApplicationError;
use common::domain::text_keys::TextKeys::{CouldNotAccessKeyring, CouldNotParseJson, CouldNotReadTokensFromKeyring, CouldNotSaveTokensToKeyring, CouldNotSerializeTokens, KeyringNotAvailable};
use engine::domain::tokens::Tokens;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStoreKeyRingAdapter;

impl TokenStoreDrivenPort for TokenStoreKeyRingAdapter {
    fn is_available(&self) -> bool {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER);
        let result = entry.unwrap().get_password();
        match result {
            Ok(_) => true,
            Err(error) => match error {
                keyring::Error::NoEntry => true,
                _ => false,
            }
        }
    }

    fn load(&self) -> Result<Option<Tokens>, ApplicationError> {
        if !self.is_available() {
            return Err(application_error!(KeyringNotAvailable));
        }
        
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
            .map_err(|e| application_error!(CouldNotAccessKeyring, e.to_string()) )?;

        let json = match entry.get_password() {
            Ok(json) => json,
            Err(error) => {
                return match error {
                    keyring::Error::NoEntry => Ok(None),
                    _ => Err(application_error!(CouldNotReadTokensFromKeyring, error.to_string()))
                }
            }
        };

        let tokens: Tokens = serde_json::from_str(&json).map_err(|e| {
            application_error!(CouldNotParseJson, e.to_string())
        })?;

        Ok(Some(tokens))
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ApplicationError> {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
            .map_err(|e| application_error!(CouldNotAccessKeyring, e.to_string()) )?;

        let json = serde_json::to_string(tokens).map_err(|e| {
            application_error!(CouldNotSerializeTokens, e.to_string())
        })?;

        entry
            .set_password(&json)
            .map_err(|e| application_error!(CouldNotSaveTokensToKeyring, e.to_string()) )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_store_key_ring_adapter_returns_ok_none_when_entry_not_found() {
        let adapter = TokenStoreKeyRingAdapter;

        if !adapter.is_available() {
            println!("Keyring not available, skipping test");
            return;
        }

        let result = adapter.load();
        dbg!(&result);
        assert_eq!(result.unwrap(), None);
    }
}
