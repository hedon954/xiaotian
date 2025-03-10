use thiserror::Error;

use crate::{models::SourceError, storage::StorageError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    AnyError(#[from] anyhow::Error),

    #[error("REPL error: {0}")]
    ReplError(#[from] reedline_repl_rs::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),

    #[error("Reference integrity error: {0}")]
    ReferenceIntegrityError(String),

    #[error("Orphaned data error: {0}")]
    OrphanedDataError(String),

    #[error("Cascade delete error: {0}")]
    CascadeDeleteError(String),

    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),
}
