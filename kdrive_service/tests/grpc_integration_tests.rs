use kdrive_service::kdrive::kdrive_service_client::KdriveServiceClient;
use kdrive_service::kdrive::Empty;
use engine::domain::engine::Engine;
use engine::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
use engine::domain::test_helpers::fake_event_bus::FakeEventBus;
use engine::domain::test_helpers::fake_token_store_adapter::{
    FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter
};
use engine::domain::test_helpers::test_store::TestStore;
use kdrive_service::grpc_handler::KdriveServiceHandler;
use tonic::transport::Server;
use std::net::SocketAddr;
use std::time::Duration;
use tonic::Request;
use kdrive_service::default_values::{default_server_addr, DEFAULT_SERVER_URL};
use kdrive_service::error::ServerError;

async fn start_test_server(addr: SocketAddr) -> Result<(), ServerError> {
    println!("test server: creating fake engine");
    let fake_engine = FakeAuthenticatorDrivenAdapter::new_default();
    let fake_ring_tokens = FakeTokenStoreRingAdapter::empty();
    let fake_file_tokens = FakeTokenStoreFileAdapter::empty();

    println!("test server: loading token store");
    let token_store: TestStore = TestStore::load(
        Some(fake_ring_tokens),
        Some(fake_file_tokens)
    )?;

    println!("test server: creating event bus");
    let fake_events = FakeEventBus::new();

    println!("test server: creating engine");
    let engine = Engine::new(fake_engine, token_store, fake_events);
    let handler = KdriveServiceHandler::new(engine);

    println!("test server: starting gRPC server on {:?}", addr);
    Server::builder()
        .add_service(kdrive_service::kdrive::kdrive_service_server::KdriveServiceServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::test]
async fn grpc_client_can_check_authentication_status() {
    // Given a running gRPC server
    let server_addr = default_server_addr();
    let server_handle = tokio::spawn(start_test_server(server_addr));

    // Wait longer for the server to actually start listening
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Wait for the client to connect
    let channel = connect_to_server().await;

    // When we create a client and check authentication
    let mut client = KdriveServiceClient::new(channel);
    let response = client.check_authentication(Request::new(Empty {})).await;

    // Then we get a response
    match response {
        Ok(resp) => {
            let auth_status = resp.into_inner();
            println!("Response: {:?}", auth_status);
            assert!(!auth_status.is_authenticated);
        }
        Err(e) => {
            panic!("gRPC call failed: {:?}", e);
        }
    }

    server_handle.abort();
}

#[tokio::test]
async fn grpc_client_can_start_auth_flow() {
    // Given a running gRPC server
    let server_addr = default_server_addr();
    let server_handle = tokio::spawn(start_test_server(server_addr));
    tokio::time::sleep(Duration::from_millis(500)).await;

    // When we connect and start auth flow
    let channel = connect_to_server().await;
    let mut client = KdriveServiceClient::new(channel);
    let response = client.start_auth_flow(Request::new(Empty {})).await;

    // Then we get a non-empty auth URL
    match response {
        Ok(resp) => {
            let url = resp.into_inner().auth_url;
            assert!(!url.is_empty());
        }
        Err(e) => panic!("gRPC call failed: {:?}", e),
    }

    server_handle.abort();
}

async fn connect_to_server() -> tonic::transport::Channel {
    for attempt in 0..30 {
        match tonic::transport::Channel::from_static(DEFAULT_SERVER_URL)
            .connect()
            .await {
            Ok(ch) => {
                println!("Connected on attempt {}", attempt);
                return ch
            }
            Err(e) => {
                println!("Connection attempt {} failed: {:?}", attempt, e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
    panic!("Failed to connect to server");
}

#[tokio::test]
async fn grpc_client_can_complete_auth_flow() {
    // Given a running gRPC server
    let server_addr = default_server_addr();
    let server_handle = tokio::spawn(start_test_server(server_addr));
    tokio::time::sleep(Duration::from_millis(500)).await;

    // When we connect and complete auth flow
    let channel = connect_to_server().await;
    let mut client = KdriveServiceClient::new(channel);
    let response = client.complete_auth_flow(Request::new(Empty {})).await;

    // Then we get success
    match response {
        Ok(resp) => {
            let result = resp.into_inner();
            assert!(result.success);
        }
        Err(e) => panic!("gRPC call failed: {:?}", e),
    }

    server_handle.abort();
}