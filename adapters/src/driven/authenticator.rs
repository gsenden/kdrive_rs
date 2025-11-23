use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::{AccessToken, AuthUrl, Client, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope, StandardRevocableToken, TokenResponse, TokenUrl};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use engine::domain::callback_endpoint::{CallbackEndpoint, ParseRedirectUrl};
use engine::domain::errors::AuthFlowError;
use engine::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;

pub struct Authenticator {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_url: RedirectUrl,
    client: Client< BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    pkce_verifier: Option<PkceCodeVerifier>,
    csrf_token: Option<CsrfToken>,

    server_handle: Option<JoinHandle<()>>,
    code_rx: Option<oneshot::Receiver<Result<String,AuthFlowError>>>,
    access_token: Option<AccessToken>,
    refresh_token: Option<RefreshToken>,
    access_token_expiry: Option<Instant>,
}
#[async_trait]
impl AuthenticatorDrivenPort for Authenticator {
    fn new(auth_url: AuthUrl, token_url: TokenUrl, client_id: ClientId, client_secret: ClientSecret, redirect_url: RedirectUrl) -> Self {
        let client = BasicClient::new(client_id.clone())
            .set_client_secret(client_secret.clone())
            .set_auth_uri(auth_url.clone())
            .set_token_uri(token_url.clone())
            .set_redirect_uri(redirect_url.clone());

        Self {
            auth_url, token_url, client_id, client_secret, redirect_url, client,
            pkce_verifier: None, csrf_token: None, server_handle: None, code_rx: None,
            access_token: None, refresh_token: None, access_token_expiry: None
        }
    }

    async fn start_initial_auth_flow(&mut self) -> Result<String, AuthFlowError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        self.pkce_verifier = Some(pkce_verifier);
        self.csrf_token = Some(csrf_token);

        let callback_endpoint = self.redirect_url.parse()?;
        let (code_tx, code_rx) = oneshot::channel::<Result<String, AuthFlowError>>();
        //let (error_tx, error_rx) = tokio::sync::mpsc::channel::<AuthFlowError>(1);
        let shared_sender = Arc::new(Mutex::new(Some(code_tx)));

        let router = self.create_router(&callback_endpoint.path, shared_sender.clone());
        let server_handle = self.start_callback_server(callback_endpoint, router).await;

        // Sla op in de struct
        self.server_handle = Some(server_handle);
        self.code_rx = Some(code_rx);
        Ok(auth_url.to_string())
    }

    async fn continue_initial_auth_flow(&mut self) -> Result<bool, AuthFlowError> {
        let pkce_verifier = match self.pkce_verifier.take() {
            Some(v) => v,
            None => return Err(AuthFlowError::FlowNotStarted),
        };

        let receiver = match self.code_rx.take() {
            Some(rx) => rx,
            None => return Err(AuthFlowError::FlowNotStarted),
        };

        // Wait for the auth code to arrive
        let code = match receiver.await {
            Ok(Ok(code)) => code,
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err(AuthFlowError::CallbackClosedUnexpectedly),
        };

        // Close the callback server
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }

        // Exchange code for tokens
        let http_client = reqwest::Client::new();
        let token_result = self
            .client
            .exchange_code(oauth2::AuthorizationCode::new(code.clone()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| AuthFlowError::TokenRequestFailed(e.to_string()))?;

        // Access token should be requested by calling get_access_token()
        self.access_token = Some(token_result.access_token().clone());

        if let Some(refresh) = token_result.refresh_token() {
            self.refresh_token = Some(refresh.clone());
        }

        Ok(true)
    }
}

impl Authenticator {

    pub async fn get_access_token(&mut self) -> Result<AccessToken, AuthFlowError> {
        if let (Some(token), Some(expiry)) =
            (&self.access_token, &self.access_token_expiry)
        {
            if Instant::now() < *expiry {
                return Ok(token.clone());
            }
        }

        let refresh_token = match &self.refresh_token {
            Some(rt) => rt,
            None => return Err(AuthFlowError::FlowNotStarted),
        };

        let http_client = reqwest::Client::new();
        let token_result = self
            .client
            .exchange_refresh_token(refresh_token)
            .request_async(&http_client)
            .await
            .map_err(|e| AuthFlowError::TokenRequestFailed(e.to_string()))?;

        self.access_token = Some(token_result.access_token().clone());
        if let Some(rt) = token_result.refresh_token() {
            self.refresh_token = Some(rt.clone());
        }
        self.access_token_expiry = token_result
            .expires_in()
            .map(|d| Instant::now() + d);

        Ok(self
            .access_token
            .clone()
            .expect("Net vernieuwd maar geen access token behouden"))
    }

    fn create_router(
        &self,
        path: &str,
        sender: Arc<Mutex<Option<oneshot::Sender<Result<String, AuthFlowError>>>>>
    ) -> Router {

        Router::new().route(
            path,
            get(move |Query(params): Query<HashMap<String, String>>| {
                async move {
                    let (status, html, result) =
                        Authenticator::handle_oauth_params(&params);

                    if let Ok(mut guard) = sender.lock() {
                        if let Some(sender) = guard.take() {
                            // Error handling is done when the receiver awaits the result
                            let _ = sender.send(result);
                        }
                    }

                    (status, Html(html)).into_response()
                }
            }),
        )
    }

    fn handle_oauth_params(params: &HashMap<String, String>) -> (StatusCode, &'static str, Result<String, AuthFlowError>) {
        match (params.get("code"), params.get("error")) {
            (Some(code), _) => (
                StatusCode::OK,
                include_str!("templates/oauth_success.html"),
                Ok(code.clone()),
            ),
            (None, Some(error)) => (
                StatusCode::BAD_REQUEST,
                include_str!("templates/oauth_configuration_error.html"),
                Err(AuthFlowError::OAuthReturnedError(error.clone())),
            ),
            (None, None) => (
                StatusCode::BAD_REQUEST,
                include_str!("templates/no_oauth_code_error.html"),
                Err(AuthFlowError::MissingAuthorizationCode),
            ),
        }
    }

    async fn start_callback_server(
        &self,
        callback_endpoint: CallbackEndpoint,
        router: Router,
    ) -> JoinHandle<()> {

        tokio::spawn(async move {
            if let Err(err) = axum_server::bind(callback_endpoint.addr)
                .serve(router.into_make_service())
                .await
            {
                eprintln!("Callback server ended: {:?}", err);
            }
        })

    }

}

// #[cfg(test)]
// mod tests {
//     use oauth2::{AccessToken, AuthUrl, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl, RefreshToken, TokenUrl};
//     use engine::domain::errors::AuthFlowError;
//     use engine::ports::driven::environment_variables_port::{EnvironmentVariablesFactoryPort, EnvironmentVariablesPort};
//     use crate::driven::authenticator::Authenticator;
//     use crate::driven::environment_variables::EnvironmentVariablesFactory;
//     use crate::test_assets::constants::*;
// 
//     #[derive(Debug, Clone)]
//     pub struct World {
//         auth_url: AuthUrl,
//         token_url: TokenUrl,
//         client_id: ClientId,
//         client_secret: ClientSecret,
//         redirect_url: RedirectUrl,
//     }
// 
//     impl World {
//         fn valid_new() -> Self {
//             World {
//                 auth_url: AuthUrl::new(String::from(VALID_TEST_AUTH_URL)).unwrap(),
//                 token_url: TokenUrl::new(String::from(VALID_TEST_TOKEN_URL)).unwrap(),
//                 client_id: ClientId::new(String::from(VALID_TEST_CLIENT_ID)),
//                 client_secret: ClientSecret::new(String::from(VALID_TEST_CLIENT_SECRET)),
//                 redirect_url: RedirectUrl::new(String::from(VALID_TEST_REDIRECT_URL)).unwrap(),
//             }
//         }
// 
//         fn valid_authenticator() -> Authenticator {
//             let world = World::valid_new();
// 
//             Authenticator::new(
//                 world.auth_url,
//                 world.token_url,
//                 world.client_id,
//                 world.client_secret,
//                 world.redirect_url,
//             )
//         }
// 
//         fn equals(&self, authenticator: Authenticator) -> bool {
//             authenticator.redirect_url == self.redirect_url &&
//                 authenticator.token_url == self.token_url &&
//                 authenticator.client_id == self.client_id &&
//                 authenticator.client_secret.secret() == self.client_secret.secret() &&
//                 authenticator.auth_url == self.auth_url
//         }
//     }
// 
//     const TEST_ACCESS_TOKEN: &str = "dummy_access_token";
//     const TEST_REFRESH_TOKEN: &str = "dummy_refresh_token";
// 
//     impl Authenticator {
//         fn equals_world(&self, world: World) -> bool {
//                 world.auth_url == self.auth_url
//                     && world.auth_url == self.client.auth_uri().clone()
//                     && world.redirect_url == self.redirect_url
//                     && world.redirect_url == self.client.redirect_uri().unwrap().clone()
//                     && world.token_url == self.token_url
//                     && world.token_url == self.client.token_uri().clone()
//                     && world.client_id == self.client_id
//                     && world.client_id == self.client.client_id().clone()
//                     && world.client_secret.secret() == self.client_secret.secret()
//         }
// 
//         async fn fake_exchange_success(&self, _code: &str, _verifier: PkceCodeVerifier)
//                                        -> Result<(AccessToken, RefreshToken), AuthFlowError>
//         {
//             Ok((
//                 AccessToken::new(TEST_ACCESS_TOKEN.to_string()),
//                 RefreshToken::new(TEST_REFRESH_TOKEN.to_string()),
//             ))
//         }
// 
//         fn tokens_are_valid(&self) -> bool {
//             let a = self.access_token.clone().unwrap().secret().clone();
//             let b = String::from(TEST_ACCESS_TOKEN);
// 
//             self.access_token.is_some()
//                 && self.access_token.clone().unwrap().secret().clone() == String::from(TEST_ACCESS_TOKEN)
//                 && self.refresh_token.is_some()
//                 && self.refresh_token.clone().unwrap().secret().clone() == String::from(TEST_REFRESH_TOKEN)
// 
//         }
//     }
// 
// 
//     #[test]
//      fn can_create_authenticator() {
//         // Given a valid authenticator configuration
//         let world = World::valid_new();
//         let expected_world = world.clone();
// 
//         // When the authenticator is created
//         let authenticator = Authenticator::new(
//             world.auth_url,
//             world.token_url,
//             world.client_id,
//             world.client_secret,
//             world.redirect_url,
//             );
// 
//         // Then it should be created successfully
//         assert!(authenticator.equals_world(expected_world));
//      }
// 
//     #[tokio::test]
//     async fn callback_listener_receives_code() {
//         let mut auth = World::valid_authenticator();
//         let _ = auth.start_initial_auth_flow().await.unwrap();
// 
//         // Geef de server eventjes tijd om te starten
//         tokio::time::sleep(std::time::Duration::from_millis(100)).await;
// 
//         // Bouw de redirect-URL vanuit je redirect_url in World
//         let redirect = auth.redirect_url.url().to_string();
//         let callback_url = format!("{}?code=test_code&state=somestate", redirect);
// 
//         // Simuleer daadwerkelijke redirect call naar je server
//         let resp = reqwest::get(&callback_url).await.unwrap();
//         assert_eq!(resp.status(), 200);
// 
//         // Haal code uit het channel, als het lukt
//         if let Some(rx) = auth.code_rx.take() {
//             let code = rx.await.unwrap().unwrap();
//             assert_eq!(code, "test_code");
//         }
// 
//         // Ruim de server taak op
//         if let Some(handle) = auth.server_handle.take() {
//             handle.abort();
//         }
//     }
// 
//     #[tokio::test]
//     async fn continue_flow_fails_if_not_started() {
//         let mut authenticator = World::valid_authenticator();
// 
//         let result = authenticator.continue_initial_auth_flow().await;
// 
//         assert!(matches!(result, Err(AuthFlowError::FlowNotStarted)));
//     }
// 
//     #[ignore]
//     #[tokio::test]
//     async fn test_token_exchange() {
//         let factory = EnvironmentVariablesFactory;
//         let env = factory.load(None).unwrap();
// 
//         let mut authenticator = Authenticator::new(
//             env.auth_url().clone(),
//             env.token_url().clone(),
//             env.client_id().clone(),
//             env.client_secret().clone(),
//             env.redirect_url().clone()
//         );
// 
//         let url = authenticator.start_initial_auth_flow().await.unwrap();
//         authenticator.continue_initial_auth_flow().await.unwrap();
//         let token = authenticator.get_access_token().await;
//         assert!(token.is_ok());
//         //let a = "DDDD";
//        // println!("{}", token);
// 
// 
// 
//     }
// }

