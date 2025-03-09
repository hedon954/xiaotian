//! Delete command handler

use std::sync::Arc;

use uuid::Uuid;

use crate::command::DeleteCommand;
use crate::process::ProcessError;
use crate::storage::Storage;

/// Handler for delete commands
pub struct DeleteHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> DeleteHandler<S> {
    /// Create a new delete handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Handle the delete command
    pub async fn handle(&self, command: DeleteCommand) -> Result<String, ProcessError> {
        match command {
            DeleteCommand::Repository { id } => self.delete_repository(id).await,
            DeleteCommand::Subscription { id } => self.delete_subscription(id).await,
        }
    }

    /// Delete a repository
    async fn delete_repository(&self, id: Uuid) -> Result<String, ProcessError> {
        // Get the repository to show what was deleted
        let repo = match self.storage.get_repository(&id).await {
            Ok(repo) => repo,
            Err(_) => {
                return Err(ProcessError::not_found(format!(
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
    async fn delete_subscription(&self, id: Uuid) -> Result<String, ProcessError> {
        // Check if subscription exists
        let subscription = match self.storage.get_subscription(&id).await? {
            Some(sub) => sub,
            None => {
                return Err(ProcessError::not_found(format!(
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
