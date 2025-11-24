use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::{AccessToken, AuthUrl, Client, ClientId, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope, StandardRevocableToken, TokenResponse, TokenUrl};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use engine::domain::callback_endpoint::{CallbackEndpoint, ParseRedirectUrl};
use engine::domain::errors::AuthFlowError;
use engine::ports::driven::authenticator_driven_port::AuthenticatorDrivenPort;

pub struct KDriveAuthenticator {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
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
impl AuthenticatorDrivenPort for KDriveAuthenticator {
    fn new(auth_url: AuthUrl, token_url: TokenUrl, client_id: ClientId, redirect_url: RedirectUrl) -> Self {
        let client = BasicClient::new(client_id.clone())
            .set_auth_uri(auth_url.clone())
            .set_token_uri(token_url.clone())
            .set_redirect_uri(redirect_url.clone());

        Self {
            auth_url, token_url, client_id, redirect_url, client,
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

impl KDriveAuthenticator {

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
                        KDriveAuthenticator::handle_oauth_params(&params);

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