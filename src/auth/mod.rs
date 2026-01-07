pub mod api_key;
pub mod file_store;
pub mod keyring_store;
pub mod oauth;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub use api_key::*;
pub use file_store::*;
pub use keyring_store::*;
pub use oauth::*;

/// Credential types supported by the CLI
/// OAuth2 is preferred, with API key as fallback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Credential {
    /// OAuth 2.0 token (PREFERRED)
    OAuth {
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<i64>,
    },
    /// Bitbucket API key (fallback for automation/CI)
    /// Note: App passwords are deprecated by Atlassian
    ApiKey {
        username: String,
        api_key: String,
    },
}

impl Credential {
    /// Get the authorization header value for API requests
    #[inline]
    pub fn auth_header(&self) -> String {
        match self {
            Credential::OAuth { access_token, .. } => {
                // Pre-allocate: "Bearer " (7) + token length
                let mut result = String::with_capacity(7 + access_token.len());
                result.push_str("Bearer ");
                result.push_str(access_token);
                result
            }
            Credential::ApiKey { username, api_key } => {
                use base64::Engine;
                // Pre-calculate capacity: "Basic " (6) + base64 encoded length
                // base64 length = ceil(input_len * 4/3)
                let input_len = username.len() + 1 + api_key.len();
                let base64_len = input_len.div_ceil(3) * 4;
                let mut result = String::with_capacity(6 + base64_len);
                result.push_str("Basic ");

                // Encode directly into a buffer to avoid intermediate String
                let mut credentials = Vec::with_capacity(input_len);
                credentials.extend_from_slice(username.as_bytes());
                credentials.push(b':');
                credentials.extend_from_slice(api_key.as_bytes());

                base64::engine::general_purpose::STANDARD.encode_string(&credentials, &mut result);
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
            Credential::ApiKey { username, .. } => Some(username),
            Credential::OAuth { .. } => None,
        }
    }
    
    /// Check if this is an OAuth credential
    #[inline]
    pub fn is_oauth(&self) -> bool {
        matches!(self, Credential::OAuth { .. })
    }
    
    /// Check if this is an API key credential
    #[inline]
    pub fn is_api_key(&self) -> bool {
        matches!(self, Credential::ApiKey { .. })
    }
}

/// Credential storage backend
enum StorageBackend {
    Keyring(KeyringStore),
    File(FileStore),
}

/// Authentication manager
pub struct AuthManager {
    backend: StorageBackend,
}

impl AuthManager {
    pub fn new() -> Result<Self> {
        // Try keyring first, fall back to file storage
        let backend = match KeyringStore::new() {
            Ok(keyring) => {
                // Test if keyring actually works by trying to read
                match keyring.get_credential() {
                    Ok(_) => {
                        // Keyring works
                        StorageBackend::Keyring(keyring)
                    }
                    Err(_) => {
                        // Keyring exists but doesn't work, use file storage
                        eprintln!("⚠️  System keyring unavailable, using file-based storage");
                        eprintln!("   Credentials will be stored in: ~/.config/bitbucket/credentials.json");
                        StorageBackend::File(FileStore::new()?)
                    }
                }
            }
            Err(_) => {
                // No keyring available, use file storage
                eprintln!("⚠️  System keyring unavailable, using file-based storage");
                eprintln!("   Credentials will be stored in: ~/.config/bitbucket/credentials.json");
                StorageBackend::File(FileStore::new()?)
            }
        };

        Ok(Self { backend })
    }

    /// Get stored credentials
    pub fn get_credentials(&self) -> Result<Option<Credential>> {
        match &self.backend {
            StorageBackend::Keyring(store) => store.get_credential(),
            StorageBackend::File(store) => store.get_credential(),
        }
    }

    /// Store credentials
    pub fn store_credentials(&self, credential: &Credential) -> Result<()> {
        match &self.backend {
            StorageBackend::Keyring(store) => store.store_credential(credential),
            StorageBackend::File(store) => store.store_credential(credential),
        }
    }

    /// Clear stored credentials
    pub fn clear_credentials(&self) -> Result<()> {
        match &self.backend {
            StorageBackend::Keyring(store) => store.delete_credential(),
            StorageBackend::File(store) => store.delete_credential(),
        }
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
