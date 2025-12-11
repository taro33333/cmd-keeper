//! Copy command implementation

use anyhow::{bail, Context, Result};
use arboard::Clipboard;
use colored::Colorize;

use crate::storage::Storage;

/// Copies a command to the clipboard by ID
pub fn execute(id: u64) -> Result<()> {
    let storage = Storage::new()?;
    let db = storage.load()?;

    // Find the command
    let entry = match db.find_by_id(id) {
        Some(e) => e,
        None => {
            bail!("Command with ID {} not found", id);
        }
    };

    // Copy to clipboard
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
    clipboard
        .set_text(&entry.command)
        .context("Failed to copy to clipboard")?;

    println!(
        "{} Command copied to clipboard!",
        "✓".green().bold()
    );
    println!("  {} {}", "ID:".dimmed(), entry.id);
    println!("  {} {}", "Command:".dimmed(), entry.command.cyan());

    Ok(())
}

