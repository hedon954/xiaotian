use thiserror::Error;

/// Errors that can occur in storage operations
#[derive(Debug, Error)]
#[allow(unused)]
pub enum StorageError {
    /// Entity not found
    #[error("Entity of type {0} not found with ID: {1}")]
    NotFound(String, String),
}
