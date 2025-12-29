use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tonic::Request;
use tokio_stream::wrappers::TcpListenerStream;

use common::kdrive::{
    Empty,
    ServerEvent,
    AuthFlowCompleted,
    server_event::Event as ServerEventKind,
    kdrive_service_server::{KdriveService, KdriveServiceServer},
};

use tonic::{Response, Status};
use std::pin::Pin;
use tonic::codegen::tokio_stream::Stream;

/// --- Fake gRPC service ---
/// This service immediately emits AuthFlowCompleted when SubscribeEvents is called
#[derive(Default)]
struct FakeKdriveService;

type EventStream =
Pin<Box<dyn Stream<Item = Result<ServerEvent, Status>> + Send>>;

#[tonic::async_trait]
impl KdriveService for FakeKdriveService {
    async fn is_authenticated(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<common::kdrive::AuthStatus>, Status> {
        Ok(Response::new(common::kdrive::AuthStatus {
            is_authenticated: false,
        }))
    }

    async fn start_initial_auth_flow(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<common::kdrive::AuthUrlResponse>, Status> {
        unreachable!("not used in this test");
    }

    async fn continue_initial_auth_flow(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }

    type SubscribeEventsStream = EventStream;

    async fn subscribe_events(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::SubscribeEventsStream>, Status> {
        let event = ServerEvent {
            event: Some(ServerEventKind::AuthFlowCompleted(
                AuthFlowCompleted {},
            )),
        };

        let stream = tokio_stream::iter(vec![Ok(event)]);
        Ok(Response::new(Box::pin(stream)))
    }
}

pub async fn start_test_server() -> (String, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let url = format!("http://{}", addr);

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(
                KdriveServiceServer::new(FakeKdriveService::default()),
            )
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    (url, handle)
}