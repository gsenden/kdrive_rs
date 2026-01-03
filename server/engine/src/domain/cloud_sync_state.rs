#[derive(Debug, PartialEq)]
pub enum CloudSyncState {
    NoMetadata,
    MetadataPresent,
    NotIndexed,
}