use thiserror::Error;

use crate::models::SourceType;

/// Errors that can occur in storage operations
#[derive(Debug, Error)]
#[allow(unused)]
pub enum StorageError {
    /// Entity not found
    #[error("Entity of type {0} not found with ID: {1}")]
    NotFound(SourceType, i32),
}
