use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::auth::{AppPasswordAuth, AuthManager};
use crate::config::Config;

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Authenticate with Bitbucket
    Login {
        /// Use app password authentication (default)
        #[arg(long, default_value = "true")]
        app_password: bool,
    },

    /// Remove stored credentials
    Logout,

    /// Show authentication status
    Status,
}

impl AuthCommands {
    pub async fn run(self) -> Result<()> {
        match self {
            AuthCommands::Login { app_password: _ } => {
                // For now, only app password auth is fully implemented
                let auth_manager = AuthManager::new()?;
                let credential = AppPasswordAuth::authenticate(&auth_manager).await?;

                // Save username to config
                if let Some(username) = credential.username() {
                    let mut config = Config::load()?;
                    config.set_username(username);
                    config.save()?;
                }

                Ok(())
            }

            AuthCommands::Logout => {
                let auth_manager = AuthManager::new()?;
                auth_manager.clear_credentials()?;

                let mut config = Config::load()?;
                config.clear_auth();
                config.save()?;

                println!("{} Logged out successfully", "✓".green());
                Ok(())
            }

            AuthCommands::Status => {
                let auth_manager = AuthManager::new()?;
                let config = Config::load()?;

                if auth_manager.is_authenticated() {
                    println!("{} Authenticated", "✓".green());

                    if let Some(username) = config.username() {
                        println!("  {} {}", "Username:".dimmed(), username);
                    }

                    if let Some(workspace) = config.default_workspace() {
                        println!("  {} {}", "Workspace:".dimmed(), workspace);
                    }

                    // Test the credentials
                    match crate::api::BitbucketClient::from_stored() {
                        Ok(client) => match client.get::<serde_json::Value>("/user").await {
                            Ok(user) => {
                                if let Some(display_name) = user.get("display_name") {
                                    println!(
                                        "  {} {}",
                                        "Display name:".dimmed(),
                                        display_name.as_str().unwrap_or("Unknown")
                                    );
                                }
                            }
                            Err(e) => {
                                println!("{} Credentials may be invalid: {}", "⚠".yellow(), e);
                            }
                        },
                        Err(e) => {
                            println!("{} Failed to create client: {}", "✗".red(), e);
                        }
                    }
                } else {
                    println!("{} Not authenticated", "✗".red());
                    println!();
                    println!("Run {} to authenticate", "bitbucket auth login".cyan());
                }

                Ok(())
            }
        }
    }
}
