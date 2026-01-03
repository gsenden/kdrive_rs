pub trait MetadataDrivenPort {
    fn has_metadata(&self) -> bool;
    fn has_index(&self) -> bool;
}