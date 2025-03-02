use thiserror::Error;

use crate::models::SourceError;
use crate::storage::StorageError;

/// Errors that can occur in REPL operations
#[derive(Debug, Error)]
pub enum ReplError {
    /// Command not found
    #[error("Command not found: {0}")]
    CommandNotFound(String),

    /// Invalid arguments
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    /// Source error
    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
