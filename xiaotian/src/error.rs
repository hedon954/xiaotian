use thiserror::Error;

use crate::{
    llm::LLMError, models::SourceError, notification::NotificationError, storage::StorageError,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    AnyError(#[from] anyhow::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),

    #[error("Notification error: {0}")]
    NotificationError(#[from] NotificationError),

    #[error("LLM error: {0}")]
    LLMError(#[from] LLMError),
}
