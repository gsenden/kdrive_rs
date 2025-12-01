use kdrive_service::error::ServerError;
use kdrive_service::start_server;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let addr = "[::1]:50051".parse()?;
    start_server(addr).await
}