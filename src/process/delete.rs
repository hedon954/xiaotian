//! Delete command handler

use std::sync::Arc;

use uuid::Uuid;

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

    /// Delete a repository
    pub async fn delete_repository(&self, id: Uuid) -> Result<String, AppError> {
        // Get the repository to show what was deleted
        let repo = match self.storage.get_repository(&id).await {
            Ok(repo) => repo,
            Err(_) => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Repository with ID {} not found",
                    id
                )));
            }
        };

        // Delete the repository
        self.storage.delete_repository(&id).await?;

        Ok(format!("Repository deleted: {}/{}", repo.owner, repo.name))
    }

    /// Delete a subscription
    pub async fn delete_subscription(&self, id: Uuid) -> Result<String, AppError> {
        // Check if subscription exists
        let subscription = match self.storage.get_subscription(&id).await? {
            Some(sub) => sub,
            None => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Subscription with ID {} not found",
                    id
                )));
            }
        };

        // Delete the subscription
        self.storage.delete_subscription(&id).await?;

        // Source ID usually includes the type prefix, extract the meaningful part
        let display_id = match subscription.source_id.split_once(':') {
            Some((_, id)) => id.to_string(),
            None => subscription.source_id.clone(),
        };

        Ok(format!("Subscription deleted: {}", display_id))
    }
}
