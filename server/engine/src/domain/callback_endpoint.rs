use std::net::SocketAddr;
use oauth2::RedirectUrl;
use url::Url;
use common::application_error;
use common::domain::errors::ApplicationError;
use common::domain::text_keys::TextKeys::InvalidRedirectUrl;


pub struct CallbackEndpoint {
    pub addr: SocketAddr,
    pub path: String,
}
pub trait ParseRedirectUrl {
    fn parse(&self) -> Result<CallbackEndpoint, ApplicationError>;
}
impl ParseRedirectUrl for RedirectUrl {
    fn parse(&self) -> Result<CallbackEndpoint, ApplicationError> {

        let parsed = Url::parse(self.as_str())
            .map_err(|_| application_error!(InvalidRedirectUrl, self.as_str()))?;

        let host = parsed
            .host_str()
            .ok_or_else(|| application_error!(InvalidRedirectUrl, self.as_str()))?;

        let port = parsed.port().unwrap_or(80);
        let path = parsed.path().to_string();

        let addr = SocketAddr::new(
            host.parse().map_err(|_| application_error!(InvalidRedirectUrl, self.as_str()))?,
            port,
        );

        Ok(CallbackEndpoint { addr, path })
    }
}