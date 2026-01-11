use anyhow::{Context, Result};
use clap::Subcommand;
use colored::Colorize;
use tabled::{Table, Tabled};

use crate::api::BitbucketClient;
use crate::models::CreateRepositoryRequest;

#[derive(Subcommand)]
pub enum RepoCommands {
    /// List repositories in a workspace
    List {
        /// Workspace slug
        workspace: String,

        /// Number of results per page
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },

    /// View repository details
    View {
        /// Repository in format workspace/repo-slug
        repo: String,

        /// Open in browser
        #[arg(short, long)]
        web: bool,
    },

    /// Clone a repository
    Clone {
        /// Repository in format workspace/repo-slug
        repo: String,

        /// Directory to clone into
        #[arg(short, long)]
        dir: Option<String>,
    },

    /// Create a new repository
    Create {
        /// Workspace slug
        workspace: String,

        /// Repository name
        name: String,

        /// Repository description
        #[arg(short, long)]
        description: Option<String>,

        /// Make repository public
        #[arg(long)]
        public: bool,

        /// Project key to add repository to
        #[arg(short, long)]
        project: Option<String>,
    },

    /// Fork a repository
    Fork {
        /// Repository to fork in format workspace/repo-slug
        repo: String,

        /// Workspace to fork into
        #[arg(short, long)]
        workspace: Option<String>,

        /// New repository name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Delete a repository
    Delete {
        /// Repository in format workspace/repo-slug
        repo: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Tabled)]
struct RepoRow {
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "DESCRIPTION")]
    description: String,
    #[tabled(rename = "PRIVATE")]
    private: String,
    #[tabled(rename = "UPDATED")]
    updated: String,
}

impl RepoCommands {
    pub async fn run(self) -> Result<()> {
        match self {
            RepoCommands::List { workspace, limit } => {
                let client = BitbucketClient::from_stored()?;
                let repos = client
                    .list_repositories(&workspace, None, Some(limit))
                    .await?;

                if repos.values.is_empty() {
                    println!("No repositories found in workspace '{}'", workspace);
                    return Ok(());
                }

                let rows: Vec<RepoRow> = repos
                    .values
                    .iter()
                    .map(|r| RepoRow {
                        name: r.full_name.clone(),
                        description: r
                            .description
                            .clone()
                            .unwrap_or_default()
                            .chars()
                            .take(40)
                            .collect::<String>(),
                        private: if r.is_private.unwrap_or(false) {
                            "Yes"
                        } else {
                            "No"
                        }
                        .to_string(),
                        updated: r
                            .updated_on
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_default(),
                    })
                    .collect();

                let table = Table::new(rows).to_string();
                println!("{}", table);

                if repos.next.is_some() {
                    println!(
                        "\n{} More repositories available. Use --limit to see more.",
                        "ℹ".blue()
                    );
                }

                Ok(())
            }

            RepoCommands::View { repo, web } => {
                let (workspace, repo_slug) = parse_repo(&repo)?;
                let client = BitbucketClient::from_stored()?;
                let repository = client.get_repository(&workspace, &repo_slug).await?;

                if web {
                    if let Some(links) = &repository.links {
                        if let Some(html) = &links.html {
                            open::that(&html.href)?;
                            println!("Opened {} in browser", html.href.cyan());
                            return Ok(());
                        }
                    }
                    anyhow::bail!("Could not find repository URL");
                }

                println!("{}", repository.full_name.bold());
                println!("{}", "─".repeat(50));

                if let Some(desc) = &repository.description {
                    if !desc.is_empty() {
                        println!("{}", desc);
                        println!();
                    }
                }

                println!(
                    "{} {}",
                    "Private:".dimmed(),
                    if repository.is_private.unwrap_or(false) {
                        "Yes"
                    } else {
                        "No"
                    }
                );
                println!(
                    "{} {}",
                    "SCM:".dimmed(),
                    repository.scm.as_deref().unwrap_or("unknown")
                );

                if let Some(lang) = &repository.language {
                    if !lang.is_empty() {
                        println!("{} {}", "Language:".dimmed(), lang);
                    }
                }

                if let Some(branch) = &repository.mainbranch {
                    println!("{} {}", "Main branch:".dimmed(), branch.name);
                }

                if let Some(size) = repository.size {
                    let size_mb = size as f64 / (1024.0 * 1024.0);
                    println!("{} {:.2} MB", "Size:".dimmed(), size_mb);
                }

                if let Some(created) = repository.created_on {
                    println!("{} {}", "Created:".dimmed(), created.format("%Y-%m-%d"));
                }

                if let Some(updated) = repository.updated_on {
                    println!("{} {}", "Updated:".dimmed(), updated.format("%Y-%m-%d"));
                }

                if let Some(links) = &repository.links {
                    println!();
                    if let Some(html) = &links.html {
                        println!("{} {}", "Web:".dimmed(), html.href.cyan());
                    }
                    if let Some(clone_links) = &links.clone {
                        for link in clone_links {
                            println!("{} {} ({})", "Clone:".dimmed(), link.href, link.name);
                        }
                    }
                }

                Ok(())
            }

            RepoCommands::Clone { repo, dir } => {
                let (workspace, repo_slug) = parse_repo(&repo)?;
                let client = BitbucketClient::from_stored()?;
                let repository = client.get_repository(&workspace, &repo_slug).await?;

                let clone_url = repository
                    .links
                    .as_ref()
                    .and_then(|l| l.clone.as_ref())
                    .and_then(|links| links.iter().find(|l| l.name == "ssh" || l.name == "https"))
                    .map(|l| &l.href)
                    .context("Could not find clone URL")?;

                let target_dir = dir.unwrap_or_else(|| repo_slug.clone());

                println!("Cloning {} into {}...", repo.cyan(), target_dir);

                let status = std::process::Command::new("git")
                    .args(["clone", clone_url, &target_dir])
                    .status()
                    .context("Failed to run git clone")?;

                if status.success() {
                    println!("{} Successfully cloned repository", "✓".green());
                } else {
                    anyhow::bail!("git clone failed");
                }

                Ok(())
            }

            RepoCommands::Create {
                workspace,
                name,
                description,
                public,
                project,
            } => {
                let client = BitbucketClient::from_stored()?;

                let slug = name.to_lowercase().replace(' ', "-");

                let request = CreateRepositoryRequest {
                    scm: "git".to_string(),
                    name: Some(name.clone()),
                    description,
                    is_private: Some(!public),
                    project: project.map(|key| crate::models::ProjectKey { key }),
                    ..Default::default()
                };

                let repository = client
                    .create_repository(&workspace, &slug, &request)
                    .await?;

                println!(
                    "{} Created repository {}",
                    "✓".green(),
                    repository.full_name.cyan()
                );

                if let Some(links) = &repository.links {
                    if let Some(html) = &links.html {
                        println!("{} {}", "URL:".dimmed(), html.href);
                    }
                }

                Ok(())
            }

            RepoCommands::Fork {
                repo,
                workspace,
                name,
            } => {
                let (src_workspace, src_repo) = parse_repo(&repo)?;
                let client = BitbucketClient::from_stored()?;

                let forked = client
                    .fork_repository(
                        &src_workspace,
                        &src_repo,
                        workspace.as_deref(),
                        name.as_deref(),
                    )
                    .await?;

                println!("{} Forked to {}", "✓".green(), forked.full_name.cyan());

                Ok(())
            }

            RepoCommands::Delete { repo, yes } => {
                let (workspace, repo_slug) = parse_repo(&repo)?;

                if !yes {
                    use dialoguer::Confirm;
                    let confirmed = Confirm::new()
                        .with_prompt(format!(
                            "Are you sure you want to delete {}? This cannot be undone!",
                            repo.red()
                        ))
                        .default(false)
                        .interact()?;

                    if !confirmed {
                        println!("Aborted");
                        return Ok(());
                    }
                }

                let client = BitbucketClient::from_stored()?;
                client.delete_repository(&workspace, &repo_slug).await?;

                println!("{} Deleted repository {}", "✓".green(), repo);

                Ok(())
            }
        }
    }
}

fn parse_repo(repo: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!(
            "Invalid repository format. Expected 'workspace/repo-slug', got '{}'",
            repo
        );
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}
