//! Delete command handler

use std::sync::Arc;

use crate::error::AppError;
use crate::storage::Storage;

/// Handler for delete commands
#[derive(Clone)]
pub struct DeleteHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> DeleteHandler<S> {
    /// Create a new delete handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Delete a repository by ID
    pub async fn delete_repository(&self, id: i32) -> Result<String, AppError> {
        self.storage.delete_repository(id).await?;
        Ok(format!("Repository with ID {} deleted successfully", id))
    }
}
