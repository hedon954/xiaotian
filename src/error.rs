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

    /// 数据引用完整性错误
    #[error("Reference integrity error: {0}")]
    ReferenceIntegrityError(String),

    /// 孤立数据错误
    #[error("Orphaned data error: {0}")]
    OrphanedDataError(String),

    /// 级联删除错误
    #[error("Cascade delete error: {0}")]
    CascadeDeleteError(String),
}
