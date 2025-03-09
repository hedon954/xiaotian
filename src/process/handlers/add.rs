//! Add command handler

use std::sync::Arc;

use uuid::Uuid;

use crate::command::AddCommand;
use crate::models::{Repository, Subscription};
use crate::process::ProcessError;
use crate::storage::Storage;

/// Handler for add commands
pub struct AddHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> AddHandler<S> {
    /// Create a new add handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Handle the add command
    pub async fn handle(&self, command: AddCommand) -> Result<String, ProcessError> {
        match command {
            AddCommand::Repository { owner, name } => self.add_repository(owner, name).await,
            AddCommand::Subscription { owner, name } => self.add_subscription(owner, name).await,
        }
    }

    /// Add a repository
    async fn add_repository(&self, owner: String, name: String) -> Result<String, ProcessError> {
        // Check if repository already exists
        match self.storage.get_repository_by_name(&owner, &name).await {
            Ok(_) => Err(ProcessError::general(format!(
                "Repository '{}/{}' already exists",
                owner, name
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
    async fn add_subscription(&self, owner: String, name: String) -> Result<String, ProcessError> {
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
