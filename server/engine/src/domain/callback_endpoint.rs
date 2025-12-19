use std::net::SocketAddr;
use oauth2::RedirectUrl;
use url::Url;
use crate::domain::errors::ServerError;
use crate::error;

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
            .map_err(|_| error!(InvalidRedirectUrl, Url => self.as_str()))?;

        let host = parsed
            .host_str()
            .ok_or_else(|| error!(InvalidRedirectUrl, Url => self.as_str()))?;

        let port = parsed.port().unwrap_or(80);
        let path = parsed.path().to_string();

        let addr = SocketAddr::new(
            host.parse().map_err(|_| error!(InvalidRedirectUrl, Url => self.as_str()))?,
            port,
        );

        Ok(CallbackEndpoint { addr, path })
    }
}