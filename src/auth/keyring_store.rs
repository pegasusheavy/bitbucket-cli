use anyhow::{Context, Result};
use keyring::Entry;

use super::Credential;

const SERVICE_NAME: &str = "bitbucket-cli";
const CREDENTIAL_KEY: &str = "credentials";

/// Secure credential storage using system keyring
pub struct KeyringStore {
    entry: Entry,
}

impl KeyringStore {
    pub fn new() -> Result<Self> {
        let entry =
            Entry::new(SERVICE_NAME, CREDENTIAL_KEY).context("Failed to create keyring entry")?;
        Ok(Self { entry })
    }

    /// Store credentials in the keyring
    pub fn store_credential(&self, credential: &Credential) -> Result<()> {
        let json = serde_json::to_string(credential).context("Failed to serialize credential")?;

        self.entry
            .set_password(&json)
            .context("Failed to store credential in keyring")?;

        Ok(())
    }

    /// Get credentials from the keyring
    pub fn get_credential(&self) -> Result<Option<Credential>> {
        match self.entry.get_password() {
            Ok(json) => {
                let credential: Credential =
                    serde_json::from_str(&json).context("Failed to parse stored credential")?;
                Ok(Some(credential))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to get credential from keyring: {}",
                e
            )),
        }
    }

    /// Delete credentials from the keyring
    pub fn delete_credential(&self) -> Result<()> {
        match self.entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(anyhow::anyhow!(
                "Failed to delete credential from keyring: {}",
                e
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note: Keyring tests require a system keyring to be available
    // and may not work in all CI environments
}
