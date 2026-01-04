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
    pub fn auth_header(&self) -> String {
        match self {
            Credential::AppPassword {
                username,
                app_password,
            } => {
                use base64::Engine;
                let credentials = format!("{}:{}", username, app_password);
                let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
                format!("Basic {}", encoded)
            }
            Credential::OAuth { access_token, .. } => {
                format!("Bearer {}", access_token)
            }
        }
    }

    /// Check if the credential needs refresh (for OAuth)
    pub fn needs_refresh(&self) -> bool {
        match self {
            Credential::OAuth {
                expires_at: Some(expires),
                ..
            } => {
                let now = chrono::Utc::now().timestamp();
                // Refresh if expiring within 5 minutes
                *expires < now + 300
            }
            _ => false,
        }
    }

    /// Get username if available
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
