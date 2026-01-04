use anyhow::{Context, Result};
use dialoguer::{Input, Password};

use super::{AuthManager, Credential};

/// App password authentication flow
pub struct AppPasswordAuth;

impl AppPasswordAuth {
    /// Run the interactive app password authentication flow
    pub async fn authenticate(auth_manager: &AuthManager) -> Result<Credential> {
        println!("\n🔐 Bitbucket App Password Authentication");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("To create an app password:");
        println!("1. Go to Bitbucket Settings → Personal Bitbucket settings");
        println!("2. Click 'App passwords' under 'Access management'");
        println!("3. Click 'Create app password'");
        println!("4. Give it a label and select required permissions");
        println!();

        let username: String = Input::new()
            .with_prompt("Bitbucket username")
            .interact_text()
            .context("Failed to read username")?;

        let app_password: String = Password::new()
            .with_prompt("App password")
            .interact()
            .context("Failed to read app password")?;

        let credential = Credential::AppPassword {
            username: username.clone(),
            app_password,
        };

        // Validate credentials by making a test API call
        Self::validate_credentials(&credential).await?;

        // Store credentials
        auth_manager.store_credentials(&credential)?;

        println!("\n✅ Successfully authenticated as {}", username);

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
            anyhow::bail!("Invalid username or app password")
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("API error ({}): {}", status, body)
        }
    }
}
