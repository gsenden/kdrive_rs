use strum_macros::{Display, EnumIter, VariantNames};

#[derive(EnumIter, Display, VariantNames, Debug)]
pub enum TextKeys {
    AuthenticateBtn,
    InvalidRedirectUrl,
    MissingRedirectUrl,
    MissingClientId,
    OAuthReturnedError,
    MissingAuthorizationCode,
    MissingStorePort,
    CouldNotCreateFolder,
    CouldNotReadTokensFromFile,
    CouldNotParseJson,
    CouldNotSerializeTokens,
    CouldNotOpenTokenFile,
    CouldNotSaveTokenFile,
    CouldNotReadTokensFromKeyring,
    CouldNotSaveTokensToKeyring,
    CouldNotAccessKeyring,
    TokenRequestFailed,
    NoRefreshTokenReceived,
    NoAccessTokenReceived,
    FlowNotStarted,
    NoConfigFolderFound
}