//! Error types for cmd-keeper
//!
//! This module defines custom error types using thiserror.

use thiserror::Error;

/// Application-specific errors
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum CmdKeeperError {
    /// Failed to determine config directory
    #[error("Could not determine config directory. Please set HOME environment variable.")]
    ConfigDirNotFound,

    /// Command entry not found by ID
    #[error("Command with ID {0} not found")]
    CommandNotFound(u64),

    /// IO error wrapper
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Clipboard error
    #[error("Clipboard error: {0}")]
    Clipboard(String),
}

/// Result type alias for cmd-keeper operations
pub type Result<T> = std::result::Result<T, CmdKeeperError>;

