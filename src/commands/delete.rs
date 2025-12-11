//! Delete command implementation

use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::storage::Storage;

/// Deletes a command by ID
pub fn execute(id: u64, force: bool) -> Result<()> {
    let storage = Storage::new()?;
    let mut db = storage.load()?;

    // Check if the command exists
    let entry = match db.find_by_id(id) {
        Some(e) => e.clone(),
        None => {
            bail!("Command with ID {} not found", id);
        }
    };

    // Confirm deletion unless --force is used
    if !force {
        println!("{}", "Command to delete:".yellow());
        println!("  {} {}", "ID:".dimmed(), entry.id);
        println!("  {} {}", "Command:".dimmed(), entry.command);
        println!("  {} {}", "Description:".dimmed(), entry.description);
        println!();

        print!("{}", "Are you sure you want to delete this? [y/N] ".yellow());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("{}", "Deletion cancelled.".dimmed());
            return Ok(());
        }
    }

    // Perform deletion
    if db.remove_by_id(id) {
        storage.save(&db)?;
        println!(
            "{} Command {} deleted successfully.",
            "✓".green(),
            id.to_string().cyan()
        );
    } else {
        bail!("Failed to delete command with ID {}", id);
    }

    Ok(())
}

