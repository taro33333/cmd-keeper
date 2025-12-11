//! Data models for cmd-keeper
//!
//! This module defines the core data structures used throughout the application.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single saved command entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEntry {
    /// Unique identifier (auto-incremented)
    pub id: u64,
    /// The actual command string
    pub command: String,
    /// Human-readable description of what the command does
    pub description: String,
    /// Optional tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
    /// Timestamp when the entry was created
    pub created_at: DateTime<Utc>,
}

impl CommandEntry {
    /// Creates a new CommandEntry with the given parameters
    pub fn new(id: u64, command: String, description: String, tags: Vec<String>) -> Self {
        Self {
            id,
            command,
            description,
            tags,
            created_at: Utc::now(),
        }
    }

    /// Returns a comma-separated string of tags
    pub fn tags_display(&self) -> String {
        if self.tags.is_empty() {
            "-".to_string()
        } else {
            self.tags.join(", ")
        }
    }
}

/// The entire database of saved commands
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandDatabase {
    /// Counter for generating unique IDs
    next_id: u64,
    /// List of all saved command entries
    pub entries: Vec<CommandEntry>,
}

impl CommandDatabase {
    /// Creates a new empty database
    pub fn new() -> Self {
        Self {
            next_id: 1,
            entries: Vec::new(),
        }
    }

    /// Adds a new command entry and returns its ID
    pub fn add(&mut self, command: String, description: String, tags: Vec<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let entry = CommandEntry::new(id, command, description, tags);
        self.entries.push(entry);
        id
    }

    /// Finds an entry by ID
    pub fn find_by_id(&self, id: u64) -> Option<&CommandEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Removes an entry by ID, returns true if found and removed
    pub fn remove_by_id(&mut self, id: u64) -> bool {
        let original_len = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < original_len
    }

    /// Searches entries by keyword (searches in command and description)
    pub fn search(&self, keyword: &str) -> Vec<&CommandEntry> {
        let keyword_lower = keyword.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.command.to_lowercase().contains(&keyword_lower)
                    || e.description.to_lowercase().contains(&keyword_lower)
                    || e.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&keyword_lower))
            })
            .collect()
    }

    /// Returns all entries
    pub fn list_all(&self) -> &[CommandEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_find() {
        let mut db = CommandDatabase::new();
        let id = db.add(
            "git status".to_string(),
            "Show git status".to_string(),
            vec!["git".to_string()],
        );

        assert_eq!(id, 1);
        let entry = db.find_by_id(1).unwrap();
        assert_eq!(entry.command, "git status");
    }

    #[test]
    fn test_search() {
        let mut db = CommandDatabase::new();
        db.add("git status".to_string(), "Show status".to_string(), vec![]);
        db.add("git log".to_string(), "Show log".to_string(), vec![]);
        db.add(
            "docker ps".to_string(),
            "List containers".to_string(),
            vec![],
        );

        let results = db.search("git");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut db = CommandDatabase::new();
        db.add("test".to_string(), "test desc".to_string(), vec![]);

        assert!(db.remove_by_id(1));
        assert!(!db.remove_by_id(1)); // Already removed
        assert!(db.entries.is_empty());
    }
}
