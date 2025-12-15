use crate::domain::configuration::Configuration;
use crate::domain::errors::ServerError;

pub trait ConfiguratorPort {
    fn load(&self) -> Result<Configuration, ServerError>;

}