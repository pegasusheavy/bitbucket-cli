// Allow dead code for API methods designed for future use
#![allow(dead_code)]

mod api;
mod auth;
mod cli;
mod config;
mod models;
mod tui;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Auth { command } => command.run().await,
        Commands::Repo { command } => command.run().await,
        Commands::Pr { command } => command.run().await,
        Commands::Issue { command } => command.run().await,
        Commands::Pipeline { command } => command.run().await,
        Commands::Tui => tui::run_tui(cli.workspace).await,
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    Ok(())
}
