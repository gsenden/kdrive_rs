use crate::domain::configuration::Configuration;
use crate::domain::errors::ConfigurationError;

pub trait ConfiguratorPort {
    fn load(&self) -> Result<Configuration, ConfigurationError>;

}