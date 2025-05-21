use std::time::Duration;

use oauth2::{basic::BasicClient, reqwest, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl};
use error::GoogleLoginError;

pub mod error;

pub struct Oauth2Client {
    client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
pub struct OAuth2UrlResult {
    pub oauth_url: String,
    pub pkce_verifier: String,
}


#[derive(Debug, Clone)]
pub struct TokenResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<Duration>,
}

impl Oauth2Client {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();
        let redirect_url = RedirectUrl::new(redirect_uri).unwrap();
        let scopes = vec![
            Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
            Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
        ];
        Oauth2Client {
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_url,
            scopes,
        }
    }

    pub fn oauth_url(&self, state: Option<String>) -> OAuth2UrlResult {
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
        let csrf_token = match state {
            Some(ref state_value) => CsrfToken::new(state_value.clone()),
            None => CsrfToken::new_random(),
        };

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone());

        let (auth_url, _csrf_token) = client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .authorize_url(|| csrf_token)
            .add_scopes(self.scopes.clone())
            .set_pkce_challenge(pkce_challenge)
            .url();

        OAuth2UrlResult {
            oauth_url: auth_url.to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
        }
    }

    pub async fn token(
        &self,
        pkce_verifier_str: &str,
        code: &str,
        timeout: Duration,
    ) -> Result<TokenResult, GoogleLoginError> {
        let pkce_verifier = oauth2::PkceCodeVerifier::new(pkce_verifier_str.to_owned());

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone());

        let http_client =
            reqwest::ClientBuilder::new().redirect(reqwest::redirect::Policy::none())
            .timeout(timeout)
            .build()?;

        let token = client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| GoogleLoginError::Token(format!("{:?}", e)))?;
        Ok(TokenResult {
            access_token: token.access_token().secret().to_string(),
            refresh_token: token.refresh_token().map(|it| it.secret().to_string()),
            expires_in: token.expires_in(),
        })
    }
}