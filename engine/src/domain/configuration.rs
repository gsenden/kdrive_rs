use oauth2::{AuthUrl, ClientId, RedirectUrl, TokenUrl};
#[derive(Debug, Clone)]
pub struct Configuration {
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub client_id: ClientId,
    pub redirect_url: RedirectUrl,
}