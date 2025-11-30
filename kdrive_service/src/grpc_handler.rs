use tonic::{Request, Response, Status};
use crate::kdrive::kdrive_service_server::KdriveService;
use crate::kdrive::{Empty, AuthStatus, AuthUrlResponse, AuthFlowResult};

pub struct KdriveServiceHandler;

#[tonic::async_trait]
impl KdriveService for KdriveServiceHandler {
    async fn check_authentication(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<AuthStatus>, Status> {
        Ok(Response::new(AuthStatus {
            is_authenticated: false,
        }))
    }

    async fn start_auth_flow(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<AuthUrlResponse>, Status> {
        Ok(Response::new(AuthUrlResponse {
            auth_url: "http://example.com".to_string(),
        }))
    }

    async fn complete_auth_flow(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<AuthFlowResult>, Status> {
        Ok(Response::new(AuthFlowResult {
            success: false,
            error: "Not implemented".to_string(),
        }))
    }
}