use crate::ports::driven::metadata_driven_port::MetadataDrivenPort;

pub struct FakeMetadataStore {}

impl FakeMetadataStore {
    pub fn new() -> Self {
        Self {}
    }    
    pub fn without_metadata() -> Self {
        Self {}
    }
}

impl MetadataDrivenPort for FakeMetadataStore {}