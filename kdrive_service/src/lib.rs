pub mod kdrive {
    tonic::include_proto!("kdrive");
}

pub mod grpc_handler;

use tonic::transport::Server;
use kdrive::kdrive_service_server::KdriveServiceServer;
use crate::grpc_handler::KdriveServiceHandler;
use std::net::SocketAddr;

pub async fn start_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    println!("kdrive_service starting on {:?}", addr);

    Server::builder()
        .add_service(KdriveServiceServer::new(KdriveServiceHandler))
        .serve(addr)
        .await?;

    Ok(())
}