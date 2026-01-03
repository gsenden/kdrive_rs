use crate::domain::engine::Engine;
use crate::domain::test_helpers::fake_authenticator_adapter::FakeAuthenticatorDrivenAdapter;
use crate::domain::test_helpers::fake_event_bus::FakeEventBus;
use crate::domain::test_helpers::fake_metadata_store::FakeMetadataStore;
use crate::domain::test_helpers::fake_token_store::FakeTokenStore;
use crate::domain::test_helpers::fake_token_store_adapter::FakeTokenStoreRingAdapter;
use crate::domain::tokens::TokenStore;

#[allow(dead_code)]
pub struct TestEngineBuilder {
    auth: FakeAuthenticatorDrivenAdapter,
    token_store: FakeTokenStore,
    event_bus: FakeEventBus,
    metadata_store: FakeMetadataStore
}

#[allow(dead_code)]
impl TestEngineBuilder {
    pub fn new() -> Self {
        Self {
            auth: FakeAuthenticatorDrivenAdapter::new_default(),
            token_store: TokenStore::load(
                Some(FakeTokenStoreRingAdapter::with_tokens()),
                None
            ).unwrap(),
            event_bus: FakeEventBus::new(),
            metadata_store: FakeMetadataStore::new()

        }
    }

    pub fn with_auth(mut self, auth: FakeAuthenticatorDrivenAdapter) -> Self {
        self.auth = auth;
        self
    }

    pub fn with_empty_token_store(mut self) -> Self {
        self.token_store = TokenStore::load(
            Some(FakeTokenStoreRingAdapter::empty()),
            None
        ).unwrap();
        self
    }

    pub fn without_metadata(mut self) -> Self {
        self.metadata_store = FakeMetadataStore::new().without_metadata();
        self
    }

    pub fn without_index(mut self) -> Self {
        self.metadata_store = FakeMetadataStore::new().without_index();
        self
    }

    pub fn build(self) -> Engine<FakeAuthenticatorDrivenAdapter, FakeTokenStore, FakeEventBus, FakeMetadataStore>
    {
        Engine::new(
            self.auth,
            self.token_store,
            self.event_bus,
            self.metadata_store
        )
    }
}