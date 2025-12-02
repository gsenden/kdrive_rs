use crate::domain::test_helpers::fake_token_store_adapter::{FakeTokenStoreFileAdapter, FakeTokenStoreRingAdapter};
use crate::domain::tokens::TokenStore;

pub type TestStore = TokenStore<FakeTokenStoreRingAdapter, FakeTokenStoreFileAdapter>;