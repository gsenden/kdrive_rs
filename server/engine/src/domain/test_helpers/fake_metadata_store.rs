use crate::ports::driven::metadata_driven_port::MetadataDrivenPort;

pub struct FakeMetadataStore {
    has_metadata: bool,
    has_index: bool,
}

impl FakeMetadataStore {
    pub fn new() -> Self {
        Self {
            has_metadata: true,
            has_index: true

        }
    }
    pub fn without_metadata(mut self) -> Self {
        self.has_metadata = false;
        self
    }

    pub fn without_index(mut self) -> Self {
        self.has_index = false;
        self
    }
}

impl MetadataDrivenPort for FakeMetadataStore {
    fn has_metadata(&self) -> bool {
        self.has_metadata
    }

    fn has_index(&self) -> bool {
        self.has_index
    }
}