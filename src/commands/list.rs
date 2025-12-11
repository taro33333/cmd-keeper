//! List command implementation

use anyhow::Result;
use colored::Colorize;
use tabled::{
    settings::{object::Columns, Modify, Style, Width},
    Table, Tabled,
};

use crate::models::CommandEntry;
use crate::storage::Storage;

/// Table row for display
#[derive(Tabled)]
struct CommandRow {
    #[tabled(rename = "ID")]
    id: u64,
    #[tabled(rename = "Command")]
    command: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Tags")]
    tags: String,
}

impl From<&CommandEntry> for CommandRow {
    fn from(entry: &CommandEntry) -> Self {
        Self {
            id: entry.id,
            command: entry.command.clone(),
            description: entry.description.clone(),
            tags: entry.tags_display(),
        }
    }
}

/// Truncates a string to the specified length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Creates a CommandRow with optional truncation
fn create_row(entry: &CommandEntry, full: bool) -> CommandRow {
    if full {
        CommandRow::from(entry)
    } else {
        CommandRow {
            id: entry.id,
            command: truncate(&entry.command, 50),
            description: truncate(&entry.description, 40),
            tags: truncate(&entry.tags_display(), 20),
        }
    }
}

/// Lists all saved commands
pub fn execute(full: bool) -> Result<()> {
    let storage = Storage::new()?;
    let db = storage.load()?;

    let entries = db.list_all();

    if entries.is_empty() {
        println!("{}", "No commands saved yet.".yellow());
        println!("Use {} to add your first command.", "cmd-keeper add".cyan());
        return Ok(());
    }

    let rows: Vec<CommandRow> = entries.iter().map(|e| create_row(e, full)).collect();

    let mut table = Table::new(rows);
    table.with(Style::rounded());

    if !full {
        table.with(Modify::new(Columns::single(1)).with(Width::truncate(50).suffix("...")));
        table.with(Modify::new(Columns::single(2)).with(Width::truncate(40).suffix("...")));
    }

    println!("{}", table);
    println!(
        "\n{} {} command(s)",
        "Total:".dimmed(),
        entries.len().to_string().cyan()
    );

    Ok(())
}
