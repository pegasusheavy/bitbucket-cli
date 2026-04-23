pub mod api_key;
pub mod file_store;
pub mod keyring_store;
pub mod oauth;

use anyhow::Result;
use base64::Engine;
use serde::{Deserialize, Serialize};

pub use api_key::*;
pub use file_store::*;
pub use keyring_store::*;
pub use oauth::*;

/// Credential types for Bitbucket authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Credential {
    /// OAuth 2.0 credentials (preferred method)
    OAuth {
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        client_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        client_secret: Option<String>,
    },
    /// API Key / HTTP Access Token (for automation/CI)
    ApiKey { username: String, api_key: String },
}

impl Credential {
    /// Get the authorization header value for API requests
    #[inline]
    pub fn auth_header(&self) -> String {
        match self {
            Credential::OAuth { access_token, .. } => {
                let mut result = String::with_capacity(7 + access_token.len());
                result.push_str("Bearer ");
                result.push_str(access_token);
                result
            }
            Credential::ApiKey { username, api_key } => {
                let basic = format!("{}:{}", username, api_key);
                let encoded = base64::engine::general_purpose::STANDARD.encode(basic.as_bytes());
                let mut result = String::with_capacity(6 + encoded.len());
                result.push_str("Basic ");
                result.push_str(&encoded);
                result
            }
        }
    }

    /// Get the credential type name for display
    #[inline]
    pub fn type_name(&self) -> &'static str {
        match self {
            Credential::OAuth { .. } => "OAuth 2.0",
            Credential::ApiKey { .. } => "API Key",
        }
    }

    /// Check if the credential needs refresh
    #[inline]
    pub fn needs_refresh(&self) -> bool {
        match self {
            Credential::OAuth {
                expires_at: Some(expires),
                ..
            } => {
                // Refresh if expiring within 5 minutes (300 seconds)
                *expires < chrono::Utc::now().timestamp() + 300
            }
            _ => false,
        }
    }

    /// Get stored OAuth consumer credentials (client_id, client_secret)
    pub fn oauth_consumer_credentials(&self) -> Option<(&str, &str)> {
        match self {
            Credential::OAuth {
                client_id: Some(id),
                client_secret: Some(secret),
                ..
            } => Some((id, secret)),
            _ => None,
        }
    }

    /// Get username (only available for API key credentials)
    pub fn username(&self) -> Option<&str> {
        match self {
            Credential::ApiKey { username, .. } => Some(username),
            _ => None,
        }
    }
}

/// Authentication manager - uses file-based credential storage
pub struct AuthManager {
    store: FileStore,
}

impl AuthManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            store: FileStore::new()?,
        })
    }

    /// Get stored credentials
    pub fn get_credentials(&self) -> Result<Option<Credential>> {
        self.store.get_credential()
    }

    /// Store credentials
    pub fn store_credentials(&self, credential: &Credential) -> Result<()> {
        self.store.store_credential(credential)
    }

    /// Clear stored credentials
    pub fn clear_credentials(&self) -> Result<()> {
        self.store.delete_credential()
    }

    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        self.get_credentials().map(|c| c.is_some()).unwrap_or(false)
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new().expect("Failed to create auth manager")
    }
}
