//! cmd-keeper - A CLI tool to save, manage, and search frequently used commands
//!
//! ## Features
//! - Add commands with descriptions and tags
//! - List all saved commands
//! - Search commands by keyword
//! - Delete commands by ID
//! - Copy commands to clipboard
//! - Interactive TUI mode (lazygit-like interface)

mod cli;
mod commands;
mod error;
mod models;
mod storage;
mod tui;

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

    // If no command specified, launch TUI mode
    let command = cli.command.unwrap_or(Commands::Tui);

    match command {
        Commands::Tui => {
            tui::run()?;
        }

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
