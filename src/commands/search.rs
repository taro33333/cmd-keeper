//! Search command implementation

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
        CommandRow {
            id: entry.id,
            command: entry.command.clone(),
            description: entry.description.clone(),
            tags: entry.tags_display(),
        }
    } else {
        CommandRow {
            id: entry.id,
            command: truncate(&entry.command, 50),
            description: truncate(&entry.description, 40),
            tags: truncate(&entry.tags_display(), 20),
        }
    }
}

/// Searches commands by keyword
pub fn execute(keyword: &str, full: bool) -> Result<()> {
    let storage = Storage::new()?;
    let db = storage.load()?;

    let results = db.search(keyword);

    if results.is_empty() {
        println!(
            "{} No commands found matching '{}'",
            "✗".red(),
            keyword.yellow()
        );
        return Ok(());
    }

    println!(
        "🔍 Found {} result(s) for '{}':\n",
        results.len().to_string().cyan(),
        keyword.yellow()
    );

    let rows: Vec<CommandRow> = results.iter().map(|e| create_row(e, full)).collect();

    let mut table = Table::new(rows);
    table.with(Style::rounded());

    if !full {
        table.with(Modify::new(Columns::single(1)).with(Width::truncate(50).suffix("...")));
        table.with(Modify::new(Columns::single(2)).with(Width::truncate(40).suffix("...")));
    }

    println!("{}", table);

    Ok(())
}
