use thiserror::Error;

use crate::{models::SourceError, storage::StorageError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    AnyError(#[from] anyhow::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),
}
