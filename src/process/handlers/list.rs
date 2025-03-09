//! List command handler

use std::sync::Arc;

use colored::Colorize;

use crate::command::ListCommand;
use crate::process::ProcessError;
use crate::storage::Storage;

/// Handler for list commands
pub struct ListHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> ListHandler<S> {
    /// Create a new list handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Handle the list command
    pub async fn handle(&self, command: ListCommand) -> Result<String, ProcessError> {
        match command {
            ListCommand::Repositories => self.list_repositories().await,
            ListCommand::Subscriptions => self.list_subscriptions().await,
            ListCommand::Updates => self.list_updates().await,
        }
    }

    /// List all repositories
    async fn list_repositories(&self) -> Result<String, ProcessError> {
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
    async fn list_subscriptions(&self) -> Result<String, ProcessError> {
        let subscriptions = self.storage.get_all_subscriptions().await?;

        if subscriptions.is_empty() {
            return Ok("No subscriptions found.".to_string());
        }

        let mut result = String::from("Subscriptions:\n");
        for sub in subscriptions {
            // Source ID usually includes the type prefix, extract the meaningful part
            let display_id = match sub.source_id.split_once(':') {
                Some((_, id)) => id,
                None => &sub.source_id,
            };

            result.push_str(&format!(
                "- {} ({})\n",
                display_id,
                sub.id.to_string().bright_blue()
            ));
        }

        Ok(result)
    }

    /// List all updates
    async fn list_updates(&self) -> Result<String, ProcessError> {
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
