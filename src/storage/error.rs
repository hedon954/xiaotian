use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in storage operations
#[derive(Debug, Error)]
#[allow(unused)]
pub enum StorageError {
    /// Entity not found
    #[error("Entity not found with ID: {0}")]
    NotFound(Uuid),

    /// Duplicate entity
    #[error("Entity already exists with ID: {0}")]
    AlreadyExists(Uuid),

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
    #[error("Cannot delete because {0} related entities exist")]
    RelatedEntitiesExist(usize),

    /// Other error
    #[error("Storage error: {0}")]
    Other(String),
}
