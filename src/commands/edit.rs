//! Edit command implementation

use anyhow::{bail, Result};
use colored::Colorize;

use crate::storage::Storage;

/// Edits an existing command in the database
pub fn execute(
    id: u64,
    command: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<()> {
    // Check if at least one field is provided
    if command.is_none() && description.is_none() && tags.is_none() {
        bail!("At least one of --command, --description, or --tags must be provided");
    }

    let storage = Storage::new()?;
    let mut db = storage.load()?;

    // Check if command exists
    let entry = db.find_by_id(id);
    if entry.is_none() {
        bail!("Command with ID {} not found", id);
    }

    // Perform update
    if db.update(id, command.clone(), description.clone(), tags.clone()) {
        storage.save(&db)?;

        println!("{}", "✓ Command updated successfully!".green().bold());
        println!("  {} {}", "ID:".dimmed(), id);

        if let Some(cmd) = command {
            println!("  {} {}", "Command:".dimmed(), cmd);
        }
        if let Some(desc) = description {
            println!("  {} {}", "Description:".dimmed(), desc);
        }
        if let Some(t) = tags {
            println!("  {} {}", "Tags:".dimmed(), t.join(", "));
        }
    } else {
        bail!("Failed to update command with ID {}", id);
    }

    Ok(())
}
