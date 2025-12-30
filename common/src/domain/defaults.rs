use const_format::concatcp;
use i18n_loader::Language;

pub const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:50051";
pub const DEFAULT_SERVER_URL: &str = concatcp!("http://",DEFAULT_SERVER_ADDRESS);
pub fn default_server_addr() -> std::net::SocketAddr {
    DEFAULT_SERVER_ADDRESS
        .parse()
        .expect("Invalid default server address")
}
pub const DOMAIN: &str = "app";
pub const DEFAULT_LANGUAGE : Language = Language::EnGb;
pub const CONNECTION_TIMEOUT_SECONDS: u64 = 2;
pub const APPLICATION_ERROR_DETAIL_FIELD_NAME: &str = "error-detail";
