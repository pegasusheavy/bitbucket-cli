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

        // Trim whitespace from token (common copy-paste issue)
        let api_key = api_key.trim().to_string();
        
        // Validate token format
        if api_key.is_empty() {
            anyhow::bail!("API key cannot be empty");
        }
        
        // Check for common Atlassian token prefixes
        if !api_key.starts_with("ATATT") && !api_key.starts_with("ATCTT") {
            println!("⚠️  Warning: Token doesn't start with expected prefix (ATATT or ATCTT)");
            println!("   This might not be a valid Bitbucket API token.");
            println!("   Token starts with: {}", &api_key.chars().take(5).collect::<String>());
        }

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

        println!("🔍 Validating credentials with Bitbucket API...");

        let response = client
            .get("https://api.bitbucket.org/2.0/user")
            .header("Authorization", credential.auth_header())
            .header("User-Agent", "bitbucket-cli/0.3.0")
            .send()
            .await
            .context("Failed to connect to Bitbucket API")?;

        let status = response.status();
        
        if status.is_success() {
            Ok(())
        } else if status == reqwest::StatusCode::UNAUTHORIZED {
            anyhow::bail!(
                "Authentication failed (401 Unauthorized).\n\n\
                Possible causes:\n\
                - Incorrect username\n\
                - Invalid or expired API token\n\
                - Token doesn't have required permissions\n\n\
                Please verify:\n\
                1. Your Bitbucket username is correct\n\
                2. Your API token is copied completely (should start with 'ATATT' or 'ATCTT')\n\
                3. Token has 'Read' permission at minimum"
            )
        } else {
            let body = response.text().await.unwrap_or_else(|_| String::from("<unable to read response>"));
            anyhow::bail!(
                "API error ({}):\n{}\n\n\
                This might indicate:\n\
                - Network connectivity issues\n\
                - Bitbucket API is unavailable\n\
                - Rate limiting",
                status, body
            )
        }
    }
}
