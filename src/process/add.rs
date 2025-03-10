//! Add command handler

use std::sync::Arc;

use crate::error::AppError;
use crate::models::{Repository, Subscription};
use crate::storage::Storage;

/// Handler for add commands
#[derive(Clone)]
pub struct AddHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> AddHandler<S> {
    /// Create a new add handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Add a repository
    pub async fn add_repository(&self, owner: String, name: String) -> Result<String, AppError> {
        // Check if repository already exists
        match self.storage.get_repository_by_name(&owner, &name).await {
            Ok(_) => Err(AppError::AnyError(anyhow::anyhow!(
                "Repository '{}/{}' already exists",
                owner,
                name
            ))),
            Err(_) => {
                // Create new repository
                let mut repo = Repository {
                    id: 0, // 让存储层分配 ID
                    owner: owner.clone(),
                    name: name.clone(),
                    url: format!("https://github.com/{}/{}", owner, name),
                };

                // Save the repository
                repo = self.storage.save_repository(repo).await?;

                Ok(format!(
                    "Repository added: {}/{} (id: {})",
                    owner, name, repo.id
                ))
            }
        }
    }

    /// Add a subscription
    pub async fn add_subscription(&self, source_id: i32) -> Result<String, AppError> {
        // get or create the repository
        let repo = match self.storage.get_repository(source_id).await? {
            Some(repo) => repo,
            None => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Repository with id {} not found",
                    source_id
                )));
            }
        };

        // create a subscription
        let subscription =
            Subscription::simple_github(repo.owner.clone(), repo.name.clone(), repo.id);

        match self.storage.save_subscription(subscription).await {
            Ok(sub) => Ok(format!(
                "Subscription added for: {}/{} (id: {})",
                repo.owner, repo.name, sub.id
            )),
            Err(e) => Err(AppError::StorageError(e)),
        }
    }
}
