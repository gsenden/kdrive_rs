use engine::ports::driven::metadata_driven_port::MetadataDrivenPort;

pub struct MetadataSqliteAdapter {}

impl MetadataSqliteAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl MetadataDrivenPort for MetadataSqliteAdapter {}