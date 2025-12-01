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
use kdrive_service::default_values::{default_server_addr};
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
    let mut channel = None;
    for attempt in 0..30 {
        match tonic::transport::Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await {
            Ok(ch) => {
                println!("Connected on attempt {}", attempt);
                channel = Some(ch);
                break;
            }
            Err(e) => {
                println!("Connection attempt {} failed: {:?}", attempt, e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    let channel = channel.expect("Failed to connect to server");

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