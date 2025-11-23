use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use engine::ports::driven::environment_variables_port::EnvironmentVariablesPort;
use engine::domain::auth::AuthUrl;
use engine::domain::errors::{AuthFlowError, ConfigurationError};
use engine::ports::driven::cloud_driven_port::CloudDrivenPort;
use oauth2::{basic::BasicClient, EndpointNotSet, EndpointSet, Scope};
use tokio::sync::{oneshot};
use engine::domain::callback_endpoint::{CallbackEndpoint, ParseRedirectUrl};

#[derive(Debug)]
pub struct KDrive {
    pub client: oauth2::Client<
        oauth2::basic::BasicErrorResponse,
        oauth2::basic::BasicTokenResponse,
        oauth2::basic::BasicTokenIntrospectionResponse,
        oauth2::StandardRevocableToken,
        oauth2::basic::BasicRevocationErrorResponse,
        EndpointSet,  // Auth endpoint
        EndpointNotSet,  // Token endpoint (not set after .set_redirect_uri)
        EndpointNotSet,  // Introspection endpoint
        EndpointNotSet,  // Revocation endpoint
        EndpointSet,  // Redirect URI
    >,
}

impl KDrive {
    fn create_callback_endpoint(&self) -> Result<CallbackEndpoint, AuthFlowError> {
        Ok(self
            .client
            .redirect_uri()
            .ok_or_else(|| AuthFlowError::MissingRedirectUrl)?
            .parse()?)
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
                        KDrive::handle_oauth_params(&params);

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

    async fn create_callback_server ( &self, callback_endpoint: CallbackEndpoint, router: Router )
        -> Result<oneshot::Receiver<()>, AuthFlowError> {

        let (ready_sender, ready_receiver) = oneshot::channel();

        tokio::spawn(async move {
            let _ = ready_sender.send(());
            axum_server::bind(callback_endpoint.addr)
                .serve(router.into_make_service())
                .await
        });

        Ok(ready_receiver)
    }


    async fn listen_for_auth_result(receiver: tokio::sync::oneshot::Receiver<Result<String, AuthFlowError>>, mut error_receiver: tokio::sync::mpsc::Receiver<AuthFlowError>) -> Result<String, AuthFlowError> {
        tokio::select! {
            Some(err) = error_receiver.recv() => Err(err),

            res = receiver => {
                match res {
                    Ok(Ok(code)) => Ok(code),
                    Ok(Err(e)) => Err(e),
                    Err(_) => Err(AuthFlowError::CallbackClosedUnexpectedly),
                }
            }
        }
    }
}


#[async_trait]
impl CloudDrivenPort for KDrive {
    fn new<E: EnvironmentVariablesPort>(environment_variables: &E) -> Result<Self, ConfigurationError> {
        let client_secret = environment_variables.client_secret().clone();
        let client =
            BasicClient::new(environment_variables.client_id().clone())
                .set_client_secret(client_secret.clone())
                .set_auth_uri(environment_variables.auth_url().clone())
                .set_token_uri(environment_variables.token_url().clone())
                .set_redirect_uri(environment_variables.redirect_url().clone());
        Ok(Self { client })
    }

    fn list_files(&self) -> Vec<String> {
       vec![String::from("test")]
    }

    fn get_authentication_url_to_be_opened_by_user(&self) -> AuthUrl {
        let (auth_url, csrf_token) =
            self
                .client
                .authorize_url(|| oauth2::CsrfToken::new_random())
                .add_scope(Scope::new("openid".to_string()))
                .url();

        AuthUrl { url: auth_url, csrf_token}
    }

    async fn get_authorization_code(&self) -> Result<String, AuthFlowError> {
        let callback_endpoint = self.create_callback_endpoint()?;

        let (sender, receiver) = oneshot::channel::<Result<String, AuthFlowError>>();
        let (_, error_receiver) = tokio::sync::mpsc::channel::<AuthFlowError>(1);

        let shared_sender = Arc::new(Mutex::new(Some(sender)));
        let router = self.create_router(&callback_endpoint.path, shared_sender.clone());

        let ready_receiver = self.create_callback_server(callback_endpoint, router).await?;
        ready_receiver.await?;

        KDrive::listen_for_auth_result(receiver, error_receiver).await
    }
}

// #[cfg(test)]
// mod tests {
//     use engine::ports::driven::cloud_driven_port::CloudDrivenPort;
//     use crate::driven::kdrive::KDrive;
//     use crate::test_assets::constants::*;
//     use crate::test_assets::fake_environment_variables::FakeEnvironmentVariables;
// 
//     #[test]
//     fn kdrive_can_get_authentication_url_to_be_opened_by_user() {
//         // Given a KDrive instance
//         let kdrive = KDrive::new(&FakeEnvironmentVariables::default()).unwrap();
// 
//         // When the auth flow is started
//         let auth_url = kdrive.get_authentication_url_to_be_opened_by_user();
// 
//         // Then the constructed auth url is correct
//         assert!(auth_url.url.as_str().starts_with(VALID_TEST_AUTH_URL));
//     }
// 
//     #[tokio::test]
//     async fn kdrive_creates_expected_callback_endpoint() {
//         // Given a KDrive instance
//         let kdrive = KDrive::new(&FakeEnvironmentVariables::default()).unwrap();
// 
//         // When the callback endpoint is created
//         let endpoint = kdrive.create_callback_endpoint().unwrap();
// 
//         // Then the constructed endpoint is correct
//         assert_eq!(endpoint.path, VALID_TEST_REDIRECT_PATH);
//     }
// 
//     use axum::{body::Body, http::Request};
//     use tokio::sync::oneshot;
//     use tower::ServiceExt;
//     use engine::domain::errors::AuthFlowError;
//     use engine::ports::driven::environment_variables_port::EnvironmentVariablesFactoryPort;
//     use crate::driven::environment_variables::EnvironmentVariablesFactory;
//     // voor `oneshot`
// 
//     #[tokio::test]
//     async fn kdrive_router_returns_ok_for_valid_code() {
//         // Given
//         let kdrive = KDrive::new(&FakeEnvironmentVariables::default()).unwrap();
//         let (tx, _rx) = oneshot::channel::<Result<String, AuthFlowError>>();
//         let shared_sender = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
//         let router = kdrive.create_router(VALID_TEST_REDIRECT_PATH, shared_sender);
//         let code = 1234;
//         let path = format!("{}?code={}", VALID_TEST_REDIRECT_PATH, code);
// 
// 
//         // When
//         let response = router
//             .oneshot(
//                 Request::builder()
//                     .uri(path)
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
// 
//         // Then
//         assert_eq!(response.status(), axum::http::StatusCode::OK);
//     }
// 
//     #[tokio::test]
//     async fn kdrive_router_returns_bad_request_without_code() {
//         // Given
//         let kdrive = KDrive::new(&FakeEnvironmentVariables::default()).unwrap();
//         let (tx, _rx) = oneshot::channel::<Result<String, AuthFlowError>>();
//         let shared_sender = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
//         let router = kdrive.create_router(VALID_TEST_REDIRECT_PATH, shared_sender);
// 
//         // When
//         let response = router
//             .oneshot(
//                 Request::builder()
//                     .uri(VALID_TEST_REDIRECT_PATH) // No code
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
// 
//         // Then
//         assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
//     }
// 
//     #[ignore = "Needs manual testing"]
//     #[tokio::test]
//     async fn connection_test() {
//         let factory = EnvironmentVariablesFactory;
//         let env_vars = factory.load(None).unwrap();
//         let kdrive = KDrive::new(&env_vars).unwrap();
// 
//         let auth = kdrive.get_authentication_url_to_be_opened_by_user();
//         println!("🔗 Open de volgende URL in je browser:\n{}", auth.url);
// 
//         let code = kdrive.get_authorization_code().await.unwrap();
// 
//         println!("✅ Code ontvangen: {}", code);
//         assert!(!code.is_empty());
//     }
// }
// 
// 
