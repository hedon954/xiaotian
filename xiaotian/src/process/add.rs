//! Add command handler

use std::sync::Arc;

use crate::error::AppError;
use crate::models::Repository;
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
                let mut repo = Repository::new(owner.clone(), name.clone());

                // Save the repository
                repo = self.storage.save_repository(repo).await?;

                Ok(format!(
                    "Repository added: {}/{} (id: {})",
                    owner, name, repo.id
                ))
            }
        }
    }
}
