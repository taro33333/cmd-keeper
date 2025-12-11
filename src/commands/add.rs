//! Add command implementation

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

/// Adds a new command to the database
pub fn execute(command: &str, description: &str, tags: Option<Vec<String>>) -> Result<()> {
    let storage = Storage::new()?;
    let mut db = storage.load()?;

    let tags = tags.unwrap_or_default();
    let id = db.add(command.to_string(), description.to_string(), tags.clone());

    storage.save(&db)?;

    println!("{}", "✓ Command saved successfully!".green().bold());
    println!("  {} {}", "ID:".dimmed(), id);
    println!("  {} {}", "Command:".dimmed(), command);
    println!("  {} {}", "Description:".dimmed(), description);
    if !tags.is_empty() {
        println!("  {} {}", "Tags:".dimmed(), tags.join(", "));
    }

    Ok(())
}
