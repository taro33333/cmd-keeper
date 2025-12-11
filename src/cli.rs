//! CLI argument parsing using clap
//!
//! Defines all subcommands and their arguments.

use clap::{Parser, Subcommand};

/// A CLI tool to save, manage, and search frequently used commands
#[derive(Parser, Debug)]
#[command(name = "cmd-keeper")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new command with description
    #[command(visible_alias = "a")]
    Add {
        /// The command to save
        #[arg(short, long)]
        command: String,

        /// Description of what the command does
        #[arg(short, long)]
        description: String,

        /// Optional tags for categorization (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// List all saved commands
    #[command(visible_alias = "ls")]
    List {
        /// Show full command without truncation
        #[arg(short, long)]
        full: bool,
    },

    /// Search commands by keyword
    #[command(visible_alias = "s")]
    Search {
        /// Keyword to search for (searches in command, description, and tags)
        keyword: String,

        /// Show full command without truncation
        #[arg(short, long)]
        full: bool,
    },

    /// Delete a command by ID
    #[command(visible_alias = "rm")]
    Delete {
        /// ID of the command to delete
        id: u64,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Copy a command to clipboard by ID
    #[command(visible_alias = "cp")]
    Copy {
        /// ID of the command to copy
        id: u64,
    },

    /// Show the path to the database file
    Path,
}

