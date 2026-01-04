pub mod auth;
pub mod issue;
pub mod pipeline;
pub mod pr;
pub mod repo;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bitbucket")]
#[command(author = "Pegasus Heavy Industries")]
#[command(version)]
#[command(about = "A command-line interface for Bitbucket Cloud", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Workspace to use (overrides config default)
    #[arg(short, long, global = true)]
    pub workspace: Option<String>,

    /// Repository to use (overrides auto-detection)
    #[arg(short, long, global = true)]
    pub repo: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage authentication with Bitbucket
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommands,
    },

    /// Manage repositories
    Repo {
        #[command(subcommand)]
        command: repo::RepoCommands,
    },

    /// Manage pull requests
    Pr {
        #[command(subcommand)]
        command: pr::PrCommands,
    },

    /// Manage issues
    Issue {
        #[command(subcommand)]
        command: issue::IssueCommands,
    },

    /// Manage pipelines
    Pipeline {
        #[command(subcommand)]
        command: pipeline::PipelineCommands,
    },

    /// Launch interactive TUI
    Tui,
}
