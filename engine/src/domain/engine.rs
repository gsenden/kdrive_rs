use crate::domain::errors::AuthFlowError;
use crate::domain::tokens::TokenStore;
use crate::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;
use crate::ports::driven::token_store_driven_port::TokenStoreDrivenPort;
use crate::ports::driving::authenticator_driving_port::AuthenticatorDrivingPort;


pub struct Engine<AP, TRP, TFP>
where
    AP: AuthenticatorDrivenPort,
    TRP: TokenStoreDrivenPort,
    TFP: TokenStoreDrivenPort,
{
    authenticator_driven_port: AP,
    token_store: TokenStore<TRP, TFP>,
    is_authenticated: bool,
}
impl<AP, TRP, TFP> Engine<AP, TRP, TFP>
where
    AP: AuthenticatorDrivenPort,
    TRP: TokenStoreDrivenPort,
    TFP: TokenStoreDrivenPort,
{
    pub fn new(
        authenticator_port: AP,
        token_store: TokenStore<TRP, TFP>,
    ) -> Self {
        // let is_authenticated = token_store.is_some();
        // Engine {
        //     authenticator_driven_port: authenticator_port,
        //     token_store,
        //     is_authenticated,
        // }
        todo!()
    }

    pub async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        self.authenticator_driven_port.start_initial_auth_flow().await
    }

    pub async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        self.authenticator_driven_port.continue_initial_auth_flow().await
    }
}

impl<AP, TRP, TFP> AuthenticatorDrivingPort for Engine<AP, TRP, TFP>
where
    AP: AuthenticatorDrivenPort,
    TRP: TokenStoreDrivenPort,
    TFP: TokenStoreDrivenPort,
{
    fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }
}

#[cfg(test)]
mod tests {
    // use crate::domain::test_helpers::fake_token_store_adapter::FakeTokenStoreRingAdapter;
    // use crate::domain::tokens::TokenStore;



    // #[test]
    // fn when_engine_is_created_without_previous_connection_it_is_not_connected() {
    //     // Given a new authenticator-driven adapter
    //     let authenticator_adapter = FakeAuthenticatorDrivenAdapter::new_default();
    //     let token_store_adapter = TokenStore::load(Some(FakeTokenStoreRingAdapter), None).unwrap();
    //
    //     // When the authenticator is created
    //     let engine = Engine::new(authenticator_adapter, Some(token_store_adapter));
    //
    //     // Then the authenticator is not connected
    //     assert_eq!(engine.is_authenticated(), false);
    // }
    //
    // #[test]
    // fn engine_without_stored_tokens_is_not_authenticated() {
    //     // Given an engine without stored tokens
    //     let adapter = FakeAuthenticatorDrivenAdapter::new_default();
    //     let engine = Engine::new(adapter);
    //
    //     // When is_authenticated is called
    //     let result = engine.is_authenticated();
    //
    //     // Then it returns false
    //     assert_eq!(result, false);
    // }
    //
    // #[test]
    // fn engine_with_stored_tokens_is_authenticated() {
    //     // Given an engine with stored tokens
    //     let adapter = FakeAuthenticatorDrivenAdapter::new_default();
    //     let token_store = TokenStore::load(
    //         Some(FakeTokenStoreRingAdapter),
    //         None
    //     ).unwrap();
    //     let engine = Engine::new(adapter, token_store);
    //
    //     // When is_authenticated is called
    //     let result = engine.is_authenticated();
    //
    //     // Then it returns true
    //     assert_eq!(result, true);
    // }


}