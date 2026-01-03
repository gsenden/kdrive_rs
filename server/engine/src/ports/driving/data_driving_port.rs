use common::domain::directory_listing::DirectoryListing;
use common::domain::errors::ApplicationError;

pub trait DataDrivingPort {
    fn get_directory_listing(&self, path: String, depth: u32) -> Result<DirectoryListing, ApplicationError>;
}