//! Error types for process module

use thiserror::Error;

use crate::models::SourceError;
use crate::storage::StorageError;

/// Errors that can occur during command processing
#[derive(Debug, Error)]
pub enum ProcessError {
    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    /// Source error
    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// General error
    #[error("Error: {0}")]
    General(String),

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),
}

impl ProcessError {
    /// Create a new general error
    pub fn general<T: Into<String>>(msg: T) -> Self {
        Self::General(msg.into())
    }

    /// Create a new not found error
    pub fn not_found<T: Into<String>>(msg: T) -> Self {
        Self::NotFound(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ProcessError::General("test error".to_string());
        assert_eq!(err.to_string(), "Error: test error");

        let err = ProcessError::not_found("test item");
        assert_eq!(err.to_string(), "Not found: test item");
    }
}
