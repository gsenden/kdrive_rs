use kdrive_service::default_values::default_server_addr;
use kdrive_service::error::ServerError;
use kdrive_service::start_server;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let addr = default_server_addr();
    start_server(addr).await
}