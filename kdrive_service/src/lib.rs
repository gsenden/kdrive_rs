// kdrive_service/src/lib.rs
pub mod kdrive {
    tonic::include_proto!("kdrive");
}

pub mod grpc_handler;

use tonic::transport::Server;
use kdrive::kdrive_service_server::KdriveServiceServer;
use crate::grpc_handler::KdriveServiceHandler;
use std::net::SocketAddr;
use adapters::driven::build_time_env_var_configurator_adapter::BuildTimeEnvVarConfiguratorPort;
// Import wat je nodig hebt voor Engine
use engine::domain::engine::Engine;
use adapters::driven::kdrive_authenticator_adapter::KDriveAuthenticator;
use adapters::driven::token_store_file_adapter::TokenStoreFileAdapter;
use adapters::driven::token_store_key_ring_adapter::TokenStoreKeyRingAdapter;
use engine::domain::test_helpers::fake_event_bus::FakeEventBus;
use engine::domain::tokens::TokenStore;
use engine::ports::driven::configurator_driven_port::ConfiguratorPort;

pub async fn start_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    println!("kdrive_service starting on {:?}", addr);

    // Maak Engine instance met concrete types
    let config = BuildTimeEnvVarConfiguratorPort.load()?;
    let authenticator = KDriveAuthenticator::new_from_config(&config);
    let token_store =
        TokenStore::load(Some(TokenStoreKeyRingAdapter), Some(TokenStoreFileAdapter));
    // let event_bus =
    //
    //
    // let token_store = TokenStoreFileAdapter;
    // let event_bus = FakeEventBus::new(); // of echter implementatie

    // let engine = Engine::new(fake_engine, token_store, event_bus);
    // let handler = KdriveServiceHandler::new(engine)
    //     .ok_or("Failed to create handler")?;
    //
    // Server::builder()
    //     .add_service(KdriveServiceServer::new(handler))
    //     .serve(addr)
    //     .await?;

    //Ok(())
    todo!()
}