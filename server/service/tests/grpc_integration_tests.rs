
use engine::domain::engine::Engine;
use engine::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
use engine::domain::test_helpers::fake_event_bus::FakeEventBus;
use engine::domain::test_helpers::fake_token_store_adapter::{
    FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter
};
use engine::domain::test_helpers::fake_token_store::FakeTokenStore;
use kdrive_service::grpc_handler::KdriveServiceHandler;
use tonic::transport::Server;
use std::net::SocketAddr;
use tonic::Request;
use tokio::net::TcpListener;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use adapters::driven::event_bus_adapter::EventBusAdapter;
use common::domain::errors::ApplicationError;
use common::kdrive::Empty;
use common::kdrive::kdrive_service_client::KdriveServiceClient;
use common::kdrive::kdrive_service_server::KdriveServiceServer;
use engine::domain::test_helpers::fake_metadata_store::FakeMetadataStore;

async fn start_test_server() -> Result<(SocketAddr, tokio::task::JoinHandle<()>), ApplicationError> {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!("test server: starting gRPC server on {:?}", addr);

    let fake_engine = FakeAuthenticatorDrivenAdapter::new_default();
    let fake_ring_tokens = FakeTokenStoreRingAdapter::empty();
    let fake_file_tokens = FakeTokenStoreFileAdapter::empty();

    let token_store: FakeTokenStore = FakeTokenStore::load(
        Some(fake_ring_tokens),
        Some(fake_file_tokens),
    )?;

    let fake_events = FakeEventBus::new();
    let fake_metadata = FakeMetadataStore::new();

    let engine = Engine::new(fake_engine, token_store, fake_events, fake_metadata);
    let event_bus = EventBusAdapter::new();
    let handler = KdriveServiceHandler::new(engine, event_bus);

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(
                KdriveServiceServer::new(
                    handler,
                ),
            )
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .expect("gRPC server failed");
    });

    Ok((addr, handle))
}
async fn connect_to_server(addr: SocketAddr) -> tonic::transport::Channel {
    let endpoint = format!("http://{}", addr);

    tonic::transport::Channel::from_shared(endpoint)
        .unwrap()
        .connect()
        .await
        .unwrap()
}

#[tokio::test]
async fn grpc_client_is_not_authenticated_initially() {
    let (addr, server_handle) = start_test_server().await.unwrap();

    let channel = connect_to_server(addr).await;
    let mut client = KdriveServiceClient::new(channel);

    let response = client
        .is_authenticated(Request::new(Empty {}))
        .await
        .unwrap();

    assert!(!response.into_inner().is_authenticated);

    server_handle.abort();
}

#[tokio::test]
async fn grpc_client_is_authenticated_after_auth_flow() {
    let (addr, server_handle) = start_test_server().await.unwrap();

    let channel = connect_to_server(addr).await;
    let mut client = KdriveServiceClient::new(channel);

    // Start auth flow
    client
        .start_initial_auth_flow(Request::new(Empty {}))
        .await
        .unwrap();

    // Complete auth flow
    client
        .continue_initial_auth_flow(Request::new(Empty {}))
        .await
        .unwrap();

    // Check authentication status
    let response = client
        .is_authenticated(Request::new(Empty {}))
        .await
        .unwrap();

    assert!(response.into_inner().is_authenticated);

    server_handle.abort();
}

#[tokio::test]
async fn grpc_client_can_start_auth_flow() {
    let (addr, server_handle) = start_test_server().await.unwrap();

    let channel = connect_to_server(addr).await;
    let mut client = KdriveServiceClient::new(channel);

    let response = client
        .start_initial_auth_flow(Request::new(Empty {}))
        .await
        .unwrap();

    assert!(!response.into_inner().auth_url.is_empty());

    server_handle.abort();
}


#[tokio::test]
async fn grpc_client_can_complete_auth_flow() {
    let (addr, server_handle) = start_test_server().await.unwrap();

    let channel = connect_to_server(addr).await;
    let mut client = KdriveServiceClient::new(channel);

    let response = client
        .continue_initial_auth_flow(Request::new(Empty {}))
        .await;

    assert!(response.is_ok());

    server_handle.abort();
}