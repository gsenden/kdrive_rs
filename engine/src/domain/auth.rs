use oauth2::url::Url;

#[derive(Debug)]
pub struct AuthUrl {
    pub url: Url,
    pub csrf_token: oauth2::CsrfToken,
}