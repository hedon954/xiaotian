//! List command handler

use std::sync::Arc;

use colored::Colorize;

use crate::{error::AppError, storage::Storage};

/// Handler for list commands
#[derive(Clone)]
pub struct ListHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> ListHandler<S> {
    /// Create a new list handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Get a repository by owner and name
    pub async fn get_repository_by_name(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<crate::models::Repository, AppError> {
        let repo = self.storage.get_repository_by_name(owner, name).await?;
        Ok(repo)
    }

    /// List all repositories
    pub async fn list_repositories(&self) -> Result<String, AppError> {
        let repositories = self.storage.get_all_repositories().await?;

        if repositories.is_empty() {
            return Ok("No repositories found.".to_string());
        }

        let mut result = String::from("Repositories:\n");
        for repo in repositories {
            result.push_str(&format!(
                "- {}/{} ({})\n",
                repo.owner,
                repo.name,
                repo.id.to_string().bright_blue()
            ));
        }

        Ok(result)
    }

    /// List all subscriptions
    pub async fn list_subscriptions(&self) -> Result<String, AppError> {
        let subscriptions = self.storage.get_all_subscriptions().await?;

        if subscriptions.is_empty() {
            return Ok("No subscriptions found.".to_string());
        }

        let mut result = String::from("Subscriptions:\n");
        for sub in subscriptions {
            result.push_str(&format!(
                "- {}: {} (source_id: {})\n",
                sub.id.to_string().bright_blue(),
                sub.name,
                sub.source_id.to_string().bright_blue(),
            ));
        }

        Ok(result)
    }

    /// List all updates
    pub async fn list_updates(&self) -> Result<String, AppError> {
        let updates = self.storage.get_all_updates().await?;

        if updates.is_empty() {
            return Ok("No updates found.".to_string());
        }

        let mut result = String::from("Updates:\n");
        for update in updates {
            result.push_str(&format!(
                "- [{}] {} ({})\n",
                update.event_date.format("%Y-%m-%d %H:%M:%S"),
                update.title,
                update.id.to_string().bright_blue()
            ));
        }

        Ok(result)
    }
}
