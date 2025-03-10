use thiserror::Error;

/// Errors that can occur in storage operations
#[derive(Debug, Error)]
#[allow(unused)]
pub enum StorageError {
    /// Entity not found
    #[error("Entity of type {0} not found with ID: {1}")]
    NotFound(String, String),

    /// Duplicate entity
    #[error("Entity of type {0} already exists with ID: {1}")]
    AlreadyExists(String, String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Reference integrity error
    #[error("Reference integrity error: {0}")]
    ReferenceIntegrityError(String),

    /// Related entities exist
    #[error("Cannot delete {0} with ID {1} because {3} {2} entities exist")]
    HasRelated(String, String, String, usize),

    /// Other error
    #[error("Storage error: {0}")]
    Other(String),
}
