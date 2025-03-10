//! Add command handler

use std::sync::Arc;

use uuid::Uuid;

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
                let repo = Repository {
                    id: Uuid::new_v4(),
                    owner: owner.clone(),
                    name: name.clone(),
                    url: format!("https://github.com/{}/{}", owner, name),
                };

                // Save the repository
                self.storage.save_repository(repo).await?;

                Ok(format!("Repository added: {}/{}", owner, name))
            }
        }
    }

    /// Add a subscription
    pub async fn add_subscription(&self, owner: String, name: String) -> Result<String, AppError> {
        // Get or create the repository
        let repo = match self.storage.get_repository_by_name(&owner, &name).await {
            Ok(repo) => repo,
            Err(_) => {
                // Create new repository
                let new_repo = Repository {
                    id: Uuid::new_v4(),
                    owner: owner.clone(),
                    name: name.clone(),
                    url: format!("https://github.com/{}/{}", owner, name),
                };

                // Save the repository and get the saved copy
                self.storage.save_repository(new_repo).await?
            }
        };

        // Create a subscription using simple_github helper method
        let subscription = Subscription::simple_github(repo.owner.clone(), repo.name.clone());

        // Save the subscription
        self.storage.save_subscription(subscription).await?;

        Ok(format!("Subscription added for: {}/{}", owner, name))
    }
}
