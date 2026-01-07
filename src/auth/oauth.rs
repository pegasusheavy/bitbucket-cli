use anyhow::{Context, Result};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, 
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use super::{AuthManager, Credential};

/// Async HTTP client for OAuth2 token exchange
async fn async_http_client(
    request: oauth2::HttpRequest,
) -> Result<oauth2::HttpResponse, reqwest::Error> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let mut request_builder = client
        .request(request.method().clone(), request.uri().to_string())
        .body(request.body().clone());

    for (name, value) in request.headers() {
        request_builder = request_builder.header(name.as_str(), value.as_bytes());
    }

    let response = request_builder.send().await?;

    let status_code = response.status();
    let headers = response.headers().to_owned();
    let body = response.bytes().await?.to_vec();

    let mut builder = oauth2::http::Response::builder()
        .status(status_code);
    
    for (name, value) in headers.iter() {
        builder = builder.header(name, value);
    }
    
    // Build the response - this should never fail with valid HTTP data
    Ok(builder.body(body).expect("Failed to build HTTP response"))
}

const BITBUCKET_AUTH_URL: &str = "https://bitbucket.org/site/oauth2/authorize";
const BITBUCKET_TOKEN_URL: &str = "https://bitbucket.org/site/oauth2/access_token";

/// OAuth 2.0 authentication flow
pub struct OAuthFlow {
    client_id: String,
    client_secret: String,
}

impl OAuthFlow {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }

    /// Run the OAuth 2.0 authentication flow
    pub async fn authenticate(&self, auth_manager: &AuthManager) -> Result<Credential> {
        println!("\n🔐 Bitbucket OAuth Authentication");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();

        // Find an available port for the callback server
        let listener =
            TcpListener::bind("127.0.0.1:0").context("Failed to bind callback server")?;
        let port = listener.local_addr()?.port();
        let redirect_url = format!("http://127.0.0.1:{}/callback", port);

        // Create OAuth client
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(AuthUrl::new(BITBUCKET_AUTH_URL.to_string())?)
            .set_token_uri(TokenUrl::new(BITBUCKET_TOKEN_URL.to_string())?)
            .set_redirect_uri(RedirectUrl::new(redirect_url.clone())?);

        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate authorization URL
        let (auth_url, csrf_token) = client
            .authorize_url(|| CsrfToken::new_random())
            .add_scope(Scope::new("repository".to_string()))
            .add_scope(Scope::new("pullrequest".to_string()))
            .add_scope(Scope::new("issue".to_string()))
            .add_scope(Scope::new("pipeline".to_string()))
            .add_scope(Scope::new("account".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        println!("Opening browser for authentication...");
        println!();

        // Try to open browser
        if open::that(auth_url.as_str()).is_err() {
            println!("Could not open browser automatically.");
            println!("Please open this URL in your browser:");
            println!();
            println!("  {}", auth_url);
            println!();
        }

        println!("Waiting for authorization...");

        // Wait for callback
        let code = Self::wait_for_callback(listener, csrf_token)?;

        println!("Authorization received, exchanging for token...");

        // Exchange code for token
        let token_response = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(&async_http_client)
            .await
            .context("Failed to exchange authorization code for token")?;

        let access_token = token_response.access_token().secret().to_string();
        let refresh_token = token_response
            .refresh_token()
            .map(|t| t.secret().to_string());
        let expires_at = token_response
            .expires_in()
            .map(|d| chrono::Utc::now().timestamp() + d.as_secs() as i64);

        let credential = Credential::OAuth {
            access_token,
            refresh_token,
            expires_at,
        };

        // Store credentials
        auth_manager.store_credentials(&credential)?;

        println!("\n✅ Successfully authenticated via OAuth");

        Ok(credential)
    }

    /// Wait for the OAuth callback and extract the authorization code
    fn wait_for_callback(
        listener: TcpListener,
        expected_csrf: CsrfToken,
    ) -> Result<AuthorizationCode> {
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };

            let mut reader = BufReader::new(&stream);
            let mut request_line = String::new();
            if reader.read_line(&mut request_line).is_err() {
                continue;
            }

            // Parse the request URL
            let Some(redirect_url) = request_line.split_whitespace().nth(1) else {
                continue;
            };

            let Ok(url) = url::Url::parse(&format!("http://localhost{}", redirect_url)) else {
                continue;
            };

            let mut code = None;
            let mut state = None;

            for (key, value) in url.query_pairs() {
                match key.as_ref() {
                    "code" => code = Some(AuthorizationCode::new(value.to_string())),
                    "state" => state = Some(CsrfToken::new(value.to_string())),
                    _ => {}
                }
            }

            // Verify CSRF token
            if let Some(ref state) = state {
                if state.secret() != expected_csrf.secret() {
                    let response = "HTTP/1.1 400 Bad Request\r\n\r\nCSRF token mismatch";
                    let _ = stream.write_all(response.as_bytes());
                    continue;
                }
            }

            // Send success response
            let response = r#"HTTP/1.1 200 OK
Content-Type: text/html

<!DOCTYPE html>
<html>
<head><title>Bitbucket CLI</title></head>
<body style="font-family: system-ui; text-align: center; padding: 50px;">
<h1>✅ Authentication Successful</h1>
<p>You can close this window and return to the terminal.</p>
</body>
</html>"#;
            let _ = stream.write_all(response.as_bytes());

            if let Some(code) = code {
                return Ok(code);
            }
        }

        anyhow::bail!("Callback server closed unexpectedly")
    }

    /// Refresh an expired OAuth token
    pub async fn refresh_token(
        &self,
        auth_manager: &AuthManager,
        refresh_token: &str,
    ) -> Result<Credential> {
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(AuthUrl::new(BITBUCKET_AUTH_URL.to_string())?)
            .set_token_uri(TokenUrl::new(BITBUCKET_TOKEN_URL.to_string())?);


        let token_response = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
            .request_async(&async_http_client)
            .await
            .context("Failed to refresh token")?;

        let access_token = token_response.access_token().secret().to_string();
        let new_refresh_token = token_response
            .refresh_token()
            .map(|t| t.secret().to_string())
            .unwrap_or_else(|| refresh_token.to_string());
        let expires_at = token_response
            .expires_in()
            .map(|d| chrono::Utc::now().timestamp() + d.as_secs() as i64);

        let credential = Credential::OAuth {
            access_token,
            refresh_token: Some(new_refresh_token),
            expires_at,
        };

        auth_manager.store_credentials(&credential)?;

        Ok(credential)
    }
}
