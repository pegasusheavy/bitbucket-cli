use anyhow::{Context, Result};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::auth::{AuthManager, Credential};
use crate::models::Paginated;

const API_BASE_URL: &str = "https://api.bitbucket.org/2.0";

/// Bitbucket API client
#[derive(Clone)]
pub struct BitbucketClient {
    client: Client,
    credential: Credential,
}

impl BitbucketClient {
    /// Create a new authenticated client
    pub fn new(credential: Credential) -> Result<Self> {
        let client = Client::builder()
            .user_agent("bitbucket-cli")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, credential })
    }

    /// Get the authorization header value
    pub fn auth_header(&self) -> String {
        self.credential.auth_header()
    }

    /// Create a client from stored credentials
    pub fn from_stored() -> Result<Self> {
        let auth_manager = AuthManager::new()?;
        let credential = auth_manager
            .get_credentials()?
            .context("Not authenticated. Run 'bitbucket auth login' first.")?;

        Self::new(credential)
    }

    /// Get the base API URL
    pub fn base_url(&self) -> &str {
        API_BASE_URL
    }

    /// Build a URL for an API endpoint
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", API_BASE_URL, path)
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .client
            .get(self.url(path))
            .header("Authorization", self.credential.auth_header())
            .send()
            .await
            .context("Request failed")?;

        self.handle_response(response).await
    }

    /// Make a GET request with query parameters
    pub async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let response = self
            .client
            .get(self.url(path))
            .header("Authorization", self.credential.auth_header())
            .query(query)
            .send()
            .await
            .context("Request failed")?;

        self.handle_response(response).await
    }

    /// Make a POST request with JSON body
    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .client
            .post(self.url(path))
            .header("Authorization", self.credential.auth_header())
            .json(body)
            .send()
            .await
            .context("Request failed")?;

        self.handle_response(response).await
    }

    /// Make a POST request without expecting a response body
    pub async fn post_no_response<B: serde::Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let response = self
            .client
            .post(self.url(path))
            .header("Authorization", self.credential.auth_header())
            .json(body)
            .send()
            .await
            .context("Request failed")?;

        self.handle_empty_response(response).await
    }

    /// Make a PUT request with JSON body
    pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .client
            .put(self.url(path))
            .header("Authorization", self.credential.auth_header())
            .json(body)
            .send()
            .await
            .context("Request failed")?;

        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let response = self
            .client
            .delete(self.url(path))
            .header("Authorization", self.credential.auth_header())
            .send()
            .await
            .context("Request failed")?;

        self.handle_empty_response(response).await
    }

    /// Fetch all pages of a paginated endpoint
    pub async fn get_all_pages<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        let mut all_items = Vec::new();
        let mut next_url: Option<String> = Some(self.url(path));

        while let Some(url) = next_url {
            let response = self
                .client
                .get(&url)
                .header("Authorization", self.credential.auth_header())
                .send()
                .await
                .context("Request failed")?;

            let page: Paginated<T> = self.handle_response(response).await?;
            all_items.extend(page.values);
            next_url = page.next;
        }

        Ok(all_items)
    }

    /// Handle API response
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response
                .json()
                .await
                .context("Failed to parse response JSON")
        } else {
            self.handle_error(status, response).await
        }
    }

    /// Handle empty response (for DELETE, etc.)
    async fn handle_empty_response(&self, response: Response) -> Result<()> {
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            self.handle_error(status, response).await
        }
    }

    /// Handle API errors
    async fn handle_error<T>(&self, status: StatusCode, response: Response) -> Result<T> {
        let body = response.text().await.unwrap_or_default();

        match status {
            StatusCode::UNAUTHORIZED => {
                anyhow::bail!("Authentication failed. Try running 'bitbucket auth login' again.")
            }
            StatusCode::FORBIDDEN => {
                anyhow::bail!("Access denied. You don't have permission to access this resource.")
            }
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Resource not found.")
            }
            StatusCode::TOO_MANY_REQUESTS => {
                anyhow::bail!("Rate limit exceeded. Please wait and try again.")
            }
            _ => {
                // Try to parse error message from response
                if let Ok(error) = serde_json::from_str::<ApiError>(&body) {
                    if let Some(msg) = error.error.message {
                        anyhow::bail!("API error: {}", msg);
                    }
                }
                anyhow::bail!("API error ({}): {}", status, body)
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(serde::Deserialize)]
struct ApiErrorDetail {
    message: Option<String>,
}
