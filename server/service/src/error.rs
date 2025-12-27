// use thiserror::Error;
// 
// #[derive(Debug, Error)]
// pub enum ServerError {
//     #[error(transparent)]
//     Domain(#[from] engine::domain::errors::ServerError),
// 
//     #[error("gRPC transport error: {0}")]
//     Transport(#[from] tonic::transport::Error),
// 
//     #[error("Invalid socket address: {0}")]
//     InvalidAddress(#[from] std::net::AddrParseError),
// }