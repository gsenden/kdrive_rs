use engine::domain::default_values::general_defaults::*;
use engine::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use common::application_error;
use common::domain::errors::ApplicationError;
use common::domain::text_keys::TextKeys::{CouldNotCreateFolder, CouldNotOpenTokenFile, CouldNotParseJson, CouldNotReadTokensFromFile, CouldNotSaveTokenFile, CouldNotSerializeTokens, NoConfigFolderFound};
use engine::domain::tokens::Tokens;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStoreFileAdapter;

impl TokenStoreDrivenPort for TokenStoreFileAdapter {
    fn is_available(&self) -> bool {
        tokens_file_path().is_ok()
    }

    fn load(&self) -> Result<Option<Tokens>, ApplicationError>
    {
        let path = tokens_file_path()?;
        let data = match fs::read_to_string(&path) {
            Ok(d) => d,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(e) => {
                return Err( application_error!(CouldNotReadTokensFromFile, e.to_string()) );
            }
        };

        let tokens: Tokens = serde_json::from_str(&data).map_err(|e| {
            application_error!(CouldNotParseJson, e.to_string())
        })?;

        Ok(Some(tokens))
    }

    fn save(&self, tokens: &Tokens) -> Result<(), ApplicationError> {
        let path = tokens_file_path()?;
        let json = serde_json::to_string_pretty(tokens).map_err(|e| {
            application_error!(CouldNotSerializeTokens, e.to_string())
        })?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;

            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .mode(0o600)
                .open(&path)
                .map_err(|e| {
                    application_error!(CouldNotOpenTokenFile, e.to_string())
                })?;

            file.write_all(json.as_bytes()).map_err(|e| {
                application_error!(CouldNotSaveTokenFile, e.to_string())
            })?;
        }

        #[cfg(not(unix))]
        {
            fs::write(&path, json).map_err(|e| {
                application_error!(CouldNotSaveTokenFile, e.to_string())
            })?;
        }

        Ok(())
    }
}

fn tokens_file_path() -> Result<PathBuf, ApplicationError> {
    let mut path = config_dir()
        .ok_or_else(|| application_error!(NoConfigFolderFound) )?;

    path.push(APPLICATION_NAME);
    fs::create_dir_all(&path)
        .map_err(|e| application_error!(CouldNotCreateFolder, e.to_string()) )?;

    path.push(TOKEN_FILE_NAME);
    Ok(path)
}

