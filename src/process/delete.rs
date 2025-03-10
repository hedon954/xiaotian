//! Delete command handler

use std::sync::Arc;
use uuid::Uuid;

use crate::error::AppError;
use crate::storage::{Storage, StorageError};

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
        match self.storage.delete_repository(id).await {
            Ok(_) => Ok(format!("Repository with ID {} deleted successfully", id)),
            Err(StorageError::HasRelated(_, id, entity_type, count)) => {
                Err(AppError::AnyError(anyhow::anyhow!(
                    "Cannot delete repository with ID {}: {} related {} entities exist",
                    id,
                    count,
                    entity_type
                )))
            }
            Err(e) => Err(AppError::StorageError(e)),
        }
    }

    /// Delete a subscription by ID
    pub async fn delete_subscription(&self, id: i32) -> Result<String, AppError> {
        match self.storage.delete_subscription(id).await {
            Ok(_) => Ok(format!("Subscription with ID {} deleted successfully", id)),
            Err(e) => Err(AppError::StorageError(e)),
        }
    }

    /// Delete a repository and all related subscriptions (cascade delete)
    pub async fn cascade_delete_repository(&self, id: i32) -> Result<String, AppError> {
        match self.storage.cascade_delete_repository(id).await {
            Ok((subs_deleted, updates_deleted)) => Ok(format!(
                "Repository with ID {} deleted successfully along with {} subscriptions and {} updates",
                id, subs_deleted, updates_deleted
            )),
            Err(e) => Err(AppError::StorageError(e)),
        }
    }

    /// Delete a subscription and all related updates (cascade delete)
    pub async fn cascade_delete_subscription(&self, id: i32) -> Result<String, AppError> {
        match self.storage.cascade_delete_subscription(id).await {
            Ok(updates_deleted) => Ok(format!(
                "Subscription with ID {} deleted successfully along with {} updates",
                id, updates_deleted
            )),
            Err(e) => Err(AppError::StorageError(e)),
        }
    }

    /// Delete an update by ID
    pub async fn delete_update(&self, id_str: &str) -> Result<String, AppError> {
        let update_id = match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return Err(AppError::InvalidIdentifier(id_str.to_string())),
        };

        match self.storage.delete_update(&update_id).await {
            Ok(_) => Ok(format!("Update with ID {} deleted successfully", update_id)),
            Err(e) => Err(AppError::StorageError(e)),
        }
    }
}
