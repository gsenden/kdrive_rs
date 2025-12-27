use common::domain::defaults::default_server_addr;
use common::domain::errors::ApplicationError;
use kdrive_service::start_server;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let addr = default_server_addr();
    start_server(addr).await
}