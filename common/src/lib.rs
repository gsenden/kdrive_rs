pub mod adapters;
pub mod domain;
pub mod ports;

pub mod kdrive {
    tonic::include_proto!("kdrive");
}

