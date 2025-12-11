//! Storage layer for cmd-keeper
//!
//! Handles loading and saving the command database to the filesystem.

use std::fs;
use std::path::PathBuf;

use crate::error::{CmdKeeperError, Result};
use crate::models::CommandDatabase;

/// Default filename for the database
const DB_FILENAME: &str = "commands.json";

/// Default directory name under config
const APP_DIR: &str = "cmd-keeper";

/// Storage handler for the command database
pub struct Storage {
    /// Path to the database file
    db_path: PathBuf,
}

impl Storage {
    /// Creates a new Storage instance with the default path
    ///
    /// The database is stored at `~/.config/cmd-keeper/commands.json` on Linux/macOS
    /// or the equivalent config directory on other platforms.
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir().ok_or(CmdKeeperError::ConfigDirNotFound)?;
        let app_dir = config_dir.join(APP_DIR);
        let db_path = app_dir.join(DB_FILENAME);

        Ok(Self { db_path })
    }

    /// Creates a Storage instance with a custom path (useful for testing)
    #[cfg(test)]
    pub fn with_path(path: PathBuf) -> Self {
        Self { db_path: path }
    }

    /// Returns the path to the database file
    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    /// Loads the command database from disk
    ///
    /// If the file doesn't exist, returns an empty database.
    pub fn load(&self) -> Result<CommandDatabase> {
        if !self.db_path.exists() {
            return Ok(CommandDatabase::new());
        }

        let content = fs::read_to_string(&self.db_path)?;
        let db: CommandDatabase = serde_json::from_str(&content)?;
        Ok(db)
    }

    /// Saves the command database to disk
    ///
    /// Creates the parent directory if it doesn't exist.
    pub fn save(&self, db: &CommandDatabase) -> Result<()> {
        // Ensure the parent directory exists
        if let Some(parent) = self.db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(db)?;
        fs::write(&self.db_path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_nonexistent() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");
        let storage = Storage::with_path(path);

        let db = storage.load().unwrap();
        assert!(db.entries.is_empty());
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");
        let storage = Storage::with_path(path);

        let mut db = CommandDatabase::new();
        db.add("test cmd".to_string(), "test desc".to_string(), vec![]);

        storage.save(&db).unwrap();

        let loaded_db = storage.load().unwrap();
        assert_eq!(loaded_db.entries.len(), 1);
        assert_eq!(loaded_db.entries[0].command, "test cmd");
    }
}
