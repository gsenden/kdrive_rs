use common::domain::errors::ApplicationError;
use crate::domain::configuration::Configuration;

pub trait ConfiguratorPort {
    fn load(&self) -> Result<Configuration, ApplicationError>;

}