use async_trait::async_trait;
use common::application_error;
use common::domain::directory_listing::DirectoryListing;
use common::domain::errors::ApplicationError;
use common::domain::text_keys::TextKeys::NotImplemented;
use crate::domain::cloud_sync_state::CloudSyncState;
use crate::domain::events::EngineEvent;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driven::event_bus_driven_port::EventBusDrivenPort;
use crate::ports::driven::metadata_driven_port::MetadataDrivenPort;
use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;
use crate::ports::driving::data_driving_port::DataDrivingPort;
use crate::ports::driving::token_store_driving_port::TokenStoreDrivingPort;

pub struct Engine<AuthPort, TokenPort, EventPort, MetadataPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort,
    MetadataPort: MetadataDrivenPort,
{
    authenticator_driven_port: AuthPort,
    token_store: TokenPort,
    event_bus: EventPort,
    #[allow(dead_code)]
    metadata_driven_port: MetadataPort,
}

impl<AuthPort, TokenPort, EventPort, MetadataPort> Engine<AuthPort, TokenPort, EventPort, MetadataPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort,
    MetadataPort: MetadataDrivenPort,
{
    pub fn new(
        authenticator_port: AuthPort,
        token_store: TokenPort,
        event_bus: EventPort,
        metadata_driven_port: MetadataPort

    ) -> Self {
        Engine {
            authenticator_driven_port: authenticator_port,
            token_store,
            event_bus,
            metadata_driven_port
        }
    }



    async fn do_auth_flow(&mut self) -> Result<(), ApplicationError> {
        self.authenticator_driven_port.continue_initial_auth_flow().await?;
        let tokens = self.authenticator_driven_port.get_tokens().await?;
        self.token_store.save_tokens(&tokens)?;
        Ok(())
    }

    fn determine_cloud_sync_state(&self) -> CloudSyncState {
        match self.metadata_driven_port.has_metadata() {
            true => CloudSyncState::MetadataPresent,
            false => CloudSyncState::NoMetadata,
        }
    }
}

#[async_trait]
impl<AuthPort, TokenPort, EventPort, MetadataPort> AuthenticatorDrivingPort for Engine<AuthPort, TokenPort, EventPort, MetadataPort>
where
    AuthPort: AuthenticatorDrivenPort + Send,
    TokenPort: TokenStoreDrivingPort + Send,
    EventPort: EventBusDrivenPort + Send,
    MetadataPort: MetadataDrivenPort + Send,
{
    fn is_authenticated(&self) -> bool {
        self.token_store.has_tokens()
    }

    async fn start_initial_auth_flow(&mut self) -> Result<String, ApplicationError> {
        self.authenticator_driven_port.start_initial_auth_flow().await
    }
    async fn continue_initial_auth_flow(&mut self) {
        let result = self.do_auth_flow().await;

        let event = match result {
            Ok(()) => EngineEvent::AuthFlowCompleted,
            Err(error) => EngineEvent::AuthFlowFailed { reason: error },
        };

        let _ = self.event_bus.emit(event);
    }
}

impl<AuthPort, TokenPort, EventPort, MetadataPort> DataDrivingPort for Engine<AuthPort, TokenPort, EventPort, MetadataPort>
where
    AuthPort: AuthenticatorDrivenPort,
    TokenPort: TokenStoreDrivingPort,
    EventPort: EventBusDrivenPort,
    MetadataPort: MetadataDrivenPort
{
    fn get_directory_listing(&self, path: String, depth: u32) -> Result<DirectoryListing, ApplicationError> {
        Err(application_error!(NotImplemented))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::cloud_sync_state::CloudSyncState;
    use crate::domain::events::EngineEvent;
    use crate::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
    use crate::domain::test_helpers::fake_metadata_store::FakeMetadataStore;
    use crate::domain::test_helpers::test_engine_builder::TestEngineBuilder;
    use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;
    use crate::ports::driving::data_driving_port::DataDrivingPort;

    #[test]
    fn user_can_view_cloud_files_for_the_first_time() {
        // Given: an authenticated user and no local cloud structure
        let engine = TestEngineBuilder::new()
            .without_index()
            .build();

        // And: a valid path and depth that will be provided by the user
        let path = "/".to_string();
        let depth = 1;


        // When: the user requests a file overview
        let result = engine.get_directory_listing(path, depth);

        // Then: a file overview is returned
        assert!(result.is_ok());
    }

    #[test]
    fn engine_reports_metadata_present_when_local_cloud_metadata_exists() {
        // Given: an engine with existing local cloud metadata
        let engine = TestEngineBuilder::new()
            .build();

        // When: determining the current cloud sync state
        let state = engine.determine_cloud_sync_state();

        // Then: the engine reports metadata is present
        assert_eq!(state, CloudSyncState::MetadataPresent);
    }

    #[test]
    fn engine_reports_no_metadata_when_no_local_cloud_state_exists() {
        // Given: an engine with no local KDrive metadata
        let engine = TestEngineBuilder::new()
            .with_metadata(FakeMetadataStore::without_metadata())
            .build();

        // When: determining the current KDrive sync state
        let state = engine.determine_cloud_sync_state();

        // Then: the engine reports that no metadata exists
        assert_eq!(state,CloudSyncState::NoMetadata);
    }


    #[test]
    fn engine_is_not_authenticated_when_token_store_has_no_tokens() {
        // Given an unauthenticated engine with a token store with no tokens
        let engine = TestEngineBuilder::new()
            .with_empty_token_store()
            .build();

        // When is_authenticated is called
        let result = engine.is_authenticated();

        // Then it returns false
        assert_eq!(result, false);
    }

    #[test]
    fn engine_is_authenticated_when_token_store_has_tokens() {
        // Given an engine with a token store that has tokens
        let engine = TestEngineBuilder::new()
            .build();

        // When is_authenticated is called
        let result = engine.is_authenticated();

        // Then it returns true
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn engine_returns_auth_url_when_starting_auth_flow() {
        // Given an unauthenticated engine
        let mut engine = TestEngineBuilder::new()
            .build();

        // When start_initial_auth_flow is called
        let result = engine.start_initial_auth_flow().await;

        // Then it returns a valid auth URL
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn engine_can_complete_full_auth_flow() {
        // Given an unauthenticated engine with a token store with no tokens
        let mut engine = TestEngineBuilder::new()
            .with_empty_token_store()
            .build();

        // When start_initial_auth_flow is called,
        // Error is ignored here because in reality a user would authenticate using the browser
        // what is needed is that the auth flow is completed
        _ = engine.start_initial_auth_flow().await;

        // And continue_initial_auth_flow is called
        engine.continue_initial_auth_flow().await;

        // Then both succeed
        assert!(engine.event_bus.get_events().contains(
            &EngineEvent::AuthFlowCompleted
        ));
    }

    #[tokio::test]
    async fn engine_emits_tokens_stored_event_when_completing_auth_flow() {
        // Given an engine with event bus
        let mut engine = TestEngineBuilder::new()
            .build();

        // When continue_initial_auth_flow is called
        _ = engine.continue_initial_auth_flow().await;

        // Then TokensStored event is emitted
        assert!(engine.event_bus.get_events().contains(&crate::domain::events::EngineEvent::AuthFlowCompleted));
    }

    #[tokio::test]
    async fn engine_emits_auth_flow_failed_event_when_auth_fails() {
        // Given an engine that will fail auth
        let adapter = FakeAuthenticatorDrivenAdapter::new_default_failing();
        let mut engine = TestEngineBuilder::new()
            .with_auth(adapter)
            .build();

        // When continue_initial_auth_flow fails
       _  = engine.continue_initial_auth_flow().await;


        // Then AuthFlowFailed event is emitted
        assert!(engine.event_bus.get_events().iter().any(|e|
            matches!(e, EngineEvent::AuthFlowFailed { .. })
        ));
    }

    #[tokio::test]
    async fn engine_persists_tokens_after_auth_flow() {
        // Given an engine with token store
        let mut engine = TestEngineBuilder::new()
            .build();

        // When auth flow completes
        _ = engine.start_initial_auth_flow().await;
        _ = engine.continue_initial_auth_flow().await;

        // Then tokens are persisted
        assert!(engine.is_authenticated());
    }
}