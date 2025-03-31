use thiserror::Error;

use crate::{
    llm::LLMError, notification::NotificationError, storage::StorageError, utils::UtilsError,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    AnyError(#[from] anyhow::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Notification error: {0}")]
    NotificationError(#[from] NotificationError),

    #[error("LLM error: {0}")]
    LLMError(#[from] LLMError),

    #[error("Utils error: {0}")]
    UtilsError(#[from] UtilsError),
}
