pub mod app_password;
pub mod keyring_store;
pub mod oauth;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub use app_password::*;
pub use keyring_store::*;

/// Credential types supported by the CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Credential {
    AppPassword {
        username: String,
        app_password: String,
    },
    OAuth {
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<i64>,
    },
}

impl Credential {
    /// Get the authorization header value for API requests
    #[inline]
    pub fn auth_header(&self) -> String {
        match self {
            Credential::AppPassword {
                username,
                app_password,
            } => {
                use base64::Engine;
                // Pre-calculate capacity: "Basic " (6) + base64 encoded length
                // base64 length = ceil(input_len * 4/3)
                let input_len = username.len() + 1 + app_password.len();
                let base64_len = input_len.div_ceil(3) * 4;
                let mut result = String::with_capacity(6 + base64_len);
                result.push_str("Basic ");

                // Encode directly into a buffer to avoid intermediate String
                let mut credentials = Vec::with_capacity(input_len);
                credentials.extend_from_slice(username.as_bytes());
                credentials.push(b':');
                credentials.extend_from_slice(app_password.as_bytes());

                base64::engine::general_purpose::STANDARD.encode_string(&credentials, &mut result);
                result
            }
            Credential::OAuth { access_token, .. } => {
                // Pre-allocate: "Bearer " (7) + token length
                let mut result = String::with_capacity(7 + access_token.len());
                result.push_str("Bearer ");
                result.push_str(access_token);
                result
            }
        }
    }

    /// Check if the credential needs refresh (for OAuth)
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

    /// Get username if available
    #[inline]
    pub fn username(&self) -> Option<&str> {
        match self {
            Credential::AppPassword { username, .. } => Some(username),
            Credential::OAuth { .. } => None,
        }
    }
}

/// Authentication manager
pub struct AuthManager {
    keyring: KeyringStore,
}

impl AuthManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            keyring: KeyringStore::new()?,
        })
    }

    /// Get stored credentials
    pub fn get_credentials(&self) -> Result<Option<Credential>> {
        self.keyring.get_credential()
    }

    /// Store credentials
    pub fn store_credentials(&self, credential: &Credential) -> Result<()> {
        self.keyring.store_credential(credential)
    }

    /// Clear stored credentials
    pub fn clear_credentials(&self) -> Result<()> {
        self.keyring.delete_credential()
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
