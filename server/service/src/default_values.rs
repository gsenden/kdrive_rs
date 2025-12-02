use const_format::concatcp;

pub const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:50051";
pub const DEFAULT_SERVER_URL: &str = concatcp!("http://",DEFAULT_SERVER_ADDRESS);
pub fn default_server_addr() -> std::net::SocketAddr {
    DEFAULT_SERVER_ADDRESS
        .parse()
        .expect("Invalid default server address")
}
