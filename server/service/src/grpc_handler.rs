use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::Stream;
use adapters::driven::event_bus_adapter::EventBusAdapter;
use engine::domain::engine::Engine;
use engine::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use engine::ports::driven::event_bus_driven_port::EventBusDrivenPort;
use engine::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;
use engine::ports::driving::token_store_driving_port::TokenStoreDrivingPort;
use common::ports::i18n_driven_port::I18nDrivenPort;
use engine::domain::events::EngineEvent;
use tokio_stream::wrappers::BroadcastStream;
use futures_util::StreamExt;
use common::kdrive::kdrive_service_server::KdriveService;
use common::kdrive::{AuthFlowCompleted, AuthStatus, AuthUrlResponse, Empty, ServerEvent};
use common::kdrive::server_event::Event as ServerEventKind;

type EventStream = Pin<Box<dyn Stream<Item = Result<ServerEvent, Status>> + Send>>;

pub struct KdriveServiceHandler<AuthPort, TokenPort, EventPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort
{
    engine: Arc<Mutex<Engine<AuthPort, TokenPort, EventPort>>>,
    event_bus: EventBusAdapter,
}

impl<AuthPort, TokenPort, EventPort> KdriveServiceHandler<AuthPort, TokenPort, EventPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort
{
    pub fn new(
        engine: Engine<AuthPort, TokenPort, EventPort>,
        event_bus: EventBusAdapter)
        -> Self
    {
        KdriveServiceHandler {
            engine: Arc::new(Mutex::new(engine)),
            event_bus,
        }
    }
}

#[tonic::async_trait]
impl<AuthPort, TokenPort, EventPort>
    KdriveService for KdriveServiceHandler<AuthPort, TokenPort, EventPort>
where
    AuthPort: AuthenticatorDrivenPort + Send + Sync + 'static,
    TokenPort: TokenStoreDrivingPort + Send + Sync + 'static,
    EventPort: EventBusDrivenPort + Send + Sync + 'static
{
    async fn is_authenticated(&self, _request: Request<Empty>)
        -> Result<Response<AuthStatus>, Status>
    {
        let engine = self.engine.lock().await;
        Ok(Response::new(AuthStatus {
            is_authenticated: engine.is_authenticated(),
        }))
    }
    async fn start_initial_auth_flow(&self, _request: Request<Empty>)
        -> Result<Response<AuthUrlResponse>, Status>
    {
        let mut engine = self.engine.lock().await;

        engine
            .start_initial_auth_flow()
            .await
            .map(|auth_url| Response::new(AuthUrlResponse { auth_url }))
            .map_err(Status::from)
    }

    async fn continue_initial_auth_flow(&self,_request: Request<Empty>)
        -> Result<Response<Empty>, Status>
    {
        let mut engine = self.engine.lock().await;

        engine
            .continue_initial_auth_flow()
            .await
            .map(|success| {
                Response::new(Empty {})
            })
            .map_err(Status::from)
    }

    type SubscribeEventsStream = EventStream;

    async fn subscribe_events(&self,_request: Request<Empty>)
        -> Result<Response<EventStream>, Status>
    {
        let receiver = self.event_bus.subscribe();

        let stream = BroadcastStream::new(receiver)
            .filter_map(|event| async move {
                match event {
                    Ok(EngineEvent::AuthFlowCompleted) => {
                        Some(Ok(ServerEvent {
                            event: Some(ServerEventKind::AuthFlowCompleted(
                                AuthFlowCompleted {},
                            )),
                        }))
                    }

                    Ok(EngineEvent::AuthFlowFailed { reason }) => {
                        Some(Ok(reason.into()))
                    }

                    Err(_) => None, // receiver lagged, drop event
                }
            });

        Ok(Response::new(Box::pin(stream)))
    }
}

#[cfg(test)]
mod tests {
    use tonic::Request;
    use adapters::driven::event_bus_adapter::EventBusAdapter;
    use common::kdrive::Empty;
    use common::kdrive::kdrive_service_server::KdriveService;
    use engine::domain::engine::Engine;
    use engine::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
    use engine::domain::test_helpers::fake_event_bus::FakeEventBus;
    use engine::domain::test_helpers::fake_token_store_adapter::{
        FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter
    };
    use engine::domain::test_helpers::test_store::TestStore;
    use crate::grpc_handler::KdriveServiceHandler;

    #[tokio::test]
    async fn check_authentication_get_engine_status() {
        // Given a handler with an authenticated engine
        let fake_engine = FakeAuthenticatorDrivenAdapter::new_default();
        let fake_ring_tokens = FakeTokenStoreRingAdapter::with_tokens();
        let fake_file_tokens = FakeTokenStoreFileAdapter::with_tokens();
        let token_store: TestStore = TestStore::load(
            Some(fake_ring_tokens),
            Some(fake_file_tokens)
        ).unwrap();
        let fake_events = FakeEventBus::new();
        let engine = Engine::new(fake_engine, token_store, fake_events);
        let event_bus = EventBusAdapter::new();
        let handler = KdriveServiceHandler::new(engine, event_bus);

        // When we call check_authentication
        let request = Request::new(Empty {});
        let response = handler.is_authenticated(request).await.unwrap();

        // Then the handler returns true (because token_store has tokens)
        assert!(response.into_inner().is_authenticated);
    }

    #[tokio::test]
    async fn start_auth_flow_returns_auth_url_from_engine() {
        // Given a handler with engine
        let fake_engine = FakeAuthenticatorDrivenAdapter::new_default();
        let fake_ring_tokens = FakeTokenStoreRingAdapter::empty();
        let fake_file_tokens = FakeTokenStoreFileAdapter::empty();
        let token_store: TestStore = TestStore::load(
            Some(fake_ring_tokens),
            Some(fake_file_tokens)
        ).unwrap();
        let fake_events = FakeEventBus::new();
        let engine = Engine::new(fake_engine, token_store, fake_events);
        let event_bus = EventBusAdapter::new();
        let handler = KdriveServiceHandler::new(engine, event_bus);

        // When we call start_auth_flow
        let request = Request::new(Empty {});
        let response = handler.start_initial_auth_flow(request).await.unwrap();

        // Then we get a non-empty auth URL
        assert!(!response.into_inner().auth_url.is_empty());
    }

    #[tokio::test]
    async fn complete_auth_flow_returns_success_when_auth_succeeds() {
        // Given a handler with engine that started auth flow
        let fake_engine = FakeAuthenticatorDrivenAdapter::new_default();
        let fake_ring_tokens = FakeTokenStoreRingAdapter::empty();
        let fake_file_tokens = FakeTokenStoreFileAdapter::empty();
        let token_store: TestStore = TestStore::load(
            Some(fake_ring_tokens),
            Some(fake_file_tokens)
        ).unwrap();
        let fake_events = FakeEventBus::new();
        let engine = Engine::new(fake_engine, token_store, fake_events);
        let event_bus = EventBusAdapter::new();
        let handler = KdriveServiceHandler::new(engine, event_bus);


        // Start the flow first
        handler.start_initial_auth_flow(Request::new(Empty {})).await.unwrap();

        // When we call complete_auth_flow
        let request = Request::new(Empty {});
        let response = handler.continue_initial_auth_flow(request).await;

        // Then success is true
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn complete_auth_flow_returns_error_when_auth_fails() {
        // Given a handler with engine
        let fake_engine = FakeAuthenticatorDrivenAdapter::new_default_failing();
        let fake_ring_tokens = FakeTokenStoreRingAdapter::empty();
        let fake_file_tokens = FakeTokenStoreFileAdapter::empty();
        let token_store: TestStore = TestStore::load(
            Some(fake_ring_tokens),
            Some(fake_file_tokens)
        ).unwrap();
        let fake_events = FakeEventBus::new();
        let engine = Engine::new(fake_engine, token_store, fake_events);
        let event_bus = EventBusAdapter::new();
        let handler = KdriveServiceHandler::new(engine, event_bus);

        // When we call complete_auth_flow without starting flow
        let request = Request::new(Empty {});
        let response = handler.continue_initial_auth_flow(request).await;

        // Then we get an error status
        assert!(response.is_err());
    }
}