// kdrive_service/src/lib.rs
pub mod kdrive {
    tonic::include_proto!("kdrive");
}

pub mod grpc_handler;
pub mod error;
pub use common as default_values;

use tonic::transport::Server;
use kdrive::kdrive_service_server::KdriveServiceServer;
use crate::grpc_handler::KdriveServiceHandler;
use std::net::SocketAddr;
use adapters::driven::build_time_env_var_configurator_adapter::BuildTimeEnvVarConfiguratorPort;
use adapters::driven::event_bus_adapter::EventBusAdapter;
use engine::domain::engine::Engine;
use adapters::driven::kdrive_authenticator_adapter::KDriveAuthenticator;
use adapters::driven::token_store_file_adapter::TokenStoreFileAdapter;
use adapters::driven::token_store_key_ring_adapter::TokenStoreKeyRingAdapter;
use engine::domain::tokens::TokenStore;
use engine::ports::driven::configurator_driven_port::ConfiguratorPort;
use crate::error::ServerError;
use common::ports::i18n_driven_port::I18nDrivenPort;
use common::adapters::i18n_embedded_adapter::I18nEmbeddedFtlAdapter;

pub async fn start_server(addr: SocketAddr) -> Result<(), ServerError> {
    println!("kdrive_service starting on {:?}", addr);

    let config = BuildTimeEnvVarConfiguratorPort.load()?;
    let authenticator = KDriveAuthenticator::new_from_config(&config);
    let token_store =
        TokenStore::load(Some(TokenStoreKeyRingAdapter), Some(TokenStoreFileAdapter))?;
    let i18n_adapter = I18nEmbeddedFtlAdapter::load();

    let event_bus = EventBusAdapter::new();

    let engine = Engine::new(
        authenticator,
        token_store,
        event_bus.clone(),
        i18n_adapter,
    );

    let handler = KdriveServiceHandler::new(
        engine,
        event_bus,
    );

    Server::builder()
        .add_service(KdriveServiceServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}