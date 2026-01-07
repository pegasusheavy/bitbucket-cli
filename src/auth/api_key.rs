use anyhow::{Context, Result};
use dialoguer::{Input, Password};

use super::{AuthManager, Credential};

/// API key authentication flow (fallback method)
/// Note: Atlassian has deprecated app passwords in favor of OAuth2
pub struct ApiKeyAuth;

impl ApiKeyAuth {
    /// Run the interactive API key authentication flow
    pub async fn authenticate(auth_manager: &AuthManager) -> Result<Credential> {
        println!("\n🔐 Bitbucket API Key Authentication");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("⚠️  Note: OAuth 2.0 is the preferred authentication method.");
        println!("   API keys are provided for automation/CI scenarios.");
        println!();
        println!("To create an API key (HTTP access token):");
        println!("1. Go to Bitbucket Settings → Personal settings");
        println!("2. Click 'HTTP access tokens' under 'Access management'");
        println!("3. Click 'Create token'");
        println!("4. Give it a label and select required permissions");
        println!();

        let username: String = Input::new()
            .with_prompt("Bitbucket username")
            .interact_text()
            .context("Failed to read username")?;

        let api_key: String = Password::new()
            .with_prompt("API key (HTTP access token)")
            .interact()
            .context("Failed to read API key")?;

        let credential = Credential::ApiKey {
            username: username.clone(),
            api_key,
        };

        // Validate credentials by making a test API call
        Self::validate_credentials(&credential).await?;

        // Store credentials
        auth_manager.store_credentials(&credential)?;

        println!("\n✅ Successfully authenticated as {}", username);
        println!("💡 Tip: Use 'bitbucket auth login --oauth' for a better experience");

        Ok(credential)
    }

    /// Validate credentials against the Bitbucket API
    async fn validate_credentials(credential: &Credential) -> Result<()> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://api.bitbucket.org/2.0/user")
            .header("Authorization", credential.auth_header())
            .send()
            .await
            .context("Failed to connect to Bitbucket API")?;

        if response.status().is_success() {
            Ok(())
        } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            anyhow::bail!("Invalid username or API key")
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("API error ({}): {}", status, body)
        }
    }
}
