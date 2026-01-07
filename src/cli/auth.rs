use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use dialoguer::{Confirm, Input};

use crate::auth::{ApiKeyAuth, AuthManager, OAuthFlow};
use crate::config::Config;

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Authenticate with Bitbucket (OAuth 2.0 preferred)
    Login {
        /// Use OAuth 2.0 authentication (recommended)
        #[arg(long)]
        oauth: bool,

        /// Use API key authentication (for automation/CI)
        #[arg(long)]
        api_key: bool,

        /// OAuth Client ID (required for OAuth)
        #[arg(long, env = "BITBUCKET_CLIENT_ID")]
        client_id: Option<String>,

        /// OAuth Client Secret (required for OAuth)
        #[arg(long, env = "BITBUCKET_CLIENT_SECRET")]
        client_secret: Option<String>,
    },

    /// Remove stored credentials
    Logout,

    /// Show authentication status
    Status,
}

impl AuthCommands {
    pub async fn run(self) -> Result<()> {
        match self {
            AuthCommands::Login {
                oauth,
                api_key,
                client_id,
                client_secret,
            } => {
                let auth_manager = AuthManager::new()?;

                // Determine authentication method
                let use_oauth = if oauth || api_key {
                    // User explicitly chose a method
                    oauth
                } else {
                    // Interactive prompt - prefer OAuth
                    println!("\n🔐 Bitbucket CLI Authentication");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!();
                    println!("Choose authentication method:");
                    println!();
                    println!("  1. {} (Recommended)", "OAuth 2.0".green().bold());
                    println!("     • More secure with token refresh");
                    println!("     • Better user experience");
                    println!("     • Requires OAuth app setup");
                    println!();
                    println!("  2. {} (Fallback)", "API Key".yellow());
                    println!("     • For automation/CI pipelines");
                    println!("     • Requires HTTP access token");
                    println!("     • No automatic refresh");
                    println!();

                    Confirm::new()
                        .with_prompt("Use OAuth 2.0?")
                        .default(true)
                        .interact()?
                };

                let credential = if use_oauth {
                    // OAuth flow
                    let client_id = client_id
                        .or_else(|| std::env::var("BITBUCKET_CLIENT_ID").ok())
                        .or_else(|| {
                            println!();
                            println!("📋 OAuth App Setup Required");
                            println!();
                            println!("To use OAuth authentication, you need to create an OAuth consumer:");
                            println!("1. Go to: https://bitbucket.org/[workspace]/workspace/settings/oauth-consumers/new");
                            println!("2. Set callback URL to: http://127.0.0.1:*/callback");
                            println!("3. Select required permissions (repository, pullrequest, issue, pipeline, account)");
                            println!("4. Copy the Client ID and Secret");
                            println!();
                            
                            Input::<String>::new()
                                .with_prompt("OAuth Client ID")
                                .interact_text()
                                .ok()
                        })
                        .ok_or_else(|| anyhow::anyhow!("OAuth Client ID is required"))?;

                    let client_secret = client_secret
                        .or_else(|| std::env::var("BITBUCKET_CLIENT_SECRET").ok())
                        .or_else(|| {
                            Input::<String>::new()
                                .with_prompt("OAuth Client Secret")
                                .interact_text()
                                .ok()
                        })
                        .ok_or_else(|| anyhow::anyhow!("OAuth Client Secret is required"))?;

                    let oauth = OAuthFlow::new(client_id, client_secret);
                    oauth.authenticate(&auth_manager).await?
                } else {
                    // API Key flow
                    ApiKeyAuth::authenticate(&auth_manager).await?
                };

                // Save username to config if available
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

                    // Show credential type
                    if let Ok(Some(credential)) = auth_manager.get_credentials() {
                        println!("  {} {}", "Method:".dimmed(), credential.type_name());
                        
                        if credential.is_oauth() && credential.needs_refresh() {
                            println!("  {} {}", "Status:".dimmed(), "Token needs refresh".yellow());
                        }
                    }

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
                    println!();
                    println!("💡 Tip: Use {} for the best experience", "--oauth".green());
                }

                Ok(())
            }
        }
    }
}
