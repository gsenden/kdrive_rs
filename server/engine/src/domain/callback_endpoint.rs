use std::net::SocketAddr;
use oauth2::RedirectUrl;
use url::Url;
use crate::domain::errors::ServerError;

pub struct CallbackEndpoint {
    pub addr: SocketAddr,
    pub path: String,
}
pub trait ParseRedirectUrl {
    fn parse(&self) -> Result<CallbackEndpoint, ServerError>;
}
impl ParseRedirectUrl for RedirectUrl {
    fn parse(&self) -> Result<CallbackEndpoint, ServerError> {
        let parsed = Url::parse(self.as_str())
            .map_err(|err| ServerError::InvalidRedirectUrl(err.to_string()))?;

        let host = parsed.host_str().unwrap_or("127.0.0.1");
        let port = parsed.port().unwrap_or(80);
        let path = parsed.path().to_string();

        let addr = SocketAddr::new(
            host.parse().unwrap_or_else(|_| std::net::Ipv4Addr::LOCALHOST.into()),
            port,
        );

        Ok(CallbackEndpoint { addr, path })
    }
}