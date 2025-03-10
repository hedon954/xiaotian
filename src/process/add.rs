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
    pub async fn add_subscription(&self, owner: String, name: String) -> Result<String, AppError> {
        // get or create the repository
        let repo = match self.storage.get_repository_by_name(&owner, &name).await {
            Ok(repo) => repo,
            Err(_) => {
                let new_repo = Repository {
                    id: 0, // 让存储层分配 ID
                    owner: owner.clone(),
                    name: name.clone(),
                    url: format!("https://github.com/{}/{}", owner, name),
                };
                self.storage.save_repository(new_repo).await?
            }
        };

        // create a subscription
        let mut subscription =
            Subscription::simple_github(repo.owner.clone(), repo.name.clone(), repo.id);

        // 让存储层分配 ID
        subscription.id = 0;

        // check if the source_id is valid (although we just created the repo, for safety)
        if !self.storage.verify_source_exists(&subscription).await? {
            return Err(AppError::ReferenceIntegrityError(format!(
                "Cannot create subscription: Source {}/{} does not exist",
                owner, name
            )));
        }

        match self.storage.save_subscription(subscription).await {
            Ok(sub) => Ok(format!(
                "Subscription added for: {}/{} (id: {})",
                owner, name, sub.id
            )),
            Err(e) => Err(AppError::StorageError(e)),
        }
    }
}
