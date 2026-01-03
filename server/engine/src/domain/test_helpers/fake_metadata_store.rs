use crate::ports::driven::metadata_driven_port::MetadataDrivenPort;

pub struct FakeMetadataStore {
    metadata_exists: bool
}

impl FakeMetadataStore {
    pub fn new() -> Self {
        Self { metadata_exists: true}
    }
    pub fn without_metadata() -> Self {
        Self { metadata_exists: false }
    }
}

impl MetadataDrivenPort for FakeMetadataStore {
    fn has_metadata(&self) -> bool {
        self.metadata_exists
    }
}