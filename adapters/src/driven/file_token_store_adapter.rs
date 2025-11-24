use engine::domain::errors::ConfigurationError;
use engine::domain::default_values::general_defaults::*;
use engine::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTokenStoreAdapter {
    access_token: String,
    refresh_token: String,
    expires_at: i64,
}

impl TokenStoreDrivenPort for FileTokenStoreAdapter {
    fn load() -> Result<Self, ConfigurationError>
    where
        Self: Sized
    {
        let path = tokens_file_path()?;

        let data = fs::read_to_string(&path).map_err(|e| {
            ConfigurationError::CouldNotReadTokensFromFile(e.to_string())
        })?;

        let tokens: FileTokenStoreAdapter =
            serde_json::from_str(&data).map_err(|e| {
                ConfigurationError::CouldNotParseJson(e.to_string())
            })?;

        Ok(tokens)
    }

    fn save(&self) -> Result<(), ConfigurationError> {
        let path = tokens_file_path()?;
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            ConfigurationError::CouldNotSerializeTokens(e.to_string())
        })?;

        // Linux/Unix: safe permissions (rw-------)
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600)
            .open(&path)
            .map_err(|e| {
                ConfigurationError::CouldNotOpenTokenFile(e.to_string())
            })?;

        file.write_all(json.as_bytes()).map_err(|e| {
            ConfigurationError::CouldNotSaveTokenFile(e.to_string())
        })?;

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

fn tokens_file_path() -> Result<PathBuf, ConfigurationError> {
    let mut path = config_dir()
        .ok_or_else(|| ConfigurationError::NoConfigFolderFound)?;

    path.push(APPLICATION_NAME);
    fs::create_dir_all(&path)
        .map_err(|e| ConfigurationError::CouldNotCreateFolder(e.to_string()))?;

    path.push(TOKEN_FILE_NAME);
    Ok(path)
}

