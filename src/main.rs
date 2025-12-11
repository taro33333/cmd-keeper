//! cmd-keeper - A CLI tool to save, manage, and search frequently used commands
//!
//! ## Features
//! - Add commands with descriptions and tags
//! - List all saved commands
//! - Search commands by keyword
//! - Delete commands by ID
//! - Copy commands to clipboard

mod cli;
mod commands;
mod error;
mod models;
mod storage;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use cli::{Cli, Commands};
use storage::Storage;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            command,
            description,
            tags,
        } => {
            commands::add(&command, &description, tags)?;
        }

        Commands::List { full } => {
            commands::list(full)?;
        }

        Commands::Search { keyword, full } => {
            commands::search(&keyword, full)?;
        }

        Commands::Delete { id, force } => {
            commands::delete(id, force)?;
        }

        Commands::Copy { id } => {
            commands::copy(id)?;
        }

        Commands::Path => {
            let storage = Storage::new()?;
            println!(
                "{} {}",
                "Database path:".dimmed(),
                storage.db_path().display().to_string().cyan()
            );
        }
    }

    Ok(())
}
