//! Delete command handler

use std::io::{Write, stdin, stdout};
use std::sync::Arc;

use nu_ansi_term::Color;
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
        let repo = match self.storage.get_repository(&id).await {
            Ok(repo) => repo,
            Err(_) => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Repository with ID {} not found",
                    id
                )));
            }
        };

        let repo_name = format!("{}/{}", repo.owner, repo.name);

        // find the related subscriptions
        let related_subs = self.storage.find_related_subscriptions(&id).await?;

        if related_subs.is_empty() {
            // if there are no related subscriptions, delete the repository directly
            self.storage.delete_repository(&id).await?;
            Ok(format!("Repository deleted: {}", repo_name))
        } else {
            // if there are related subscriptions, prompt the user and request confirmation
            println!(
                "This repository has {} related subscriptions:",
                related_subs.len()
            );
            for sub in &related_subs {
                println!("- {} ({})", sub.name, Color::Blue.paint(sub.id.to_string()));
            }

            println!(
                "\nDeleting this repository will also delete these subscriptions and their updates."
            );
            print!("Do you want to proceed? (y/N): ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() == "y" {
                let (deleted_subs, deleted_updates) =
                    self.storage.cascade_delete_repository(&id).await?;

                Ok(format!(
                    "Repository {} deleted along with {} subscriptions and {} updates.",
                    Color::Green.paint(&repo_name),
                    Color::Green.paint(deleted_subs.to_string()),
                    Color::Green.paint(deleted_updates.to_string())
                ))
            } else {
                Ok("Deletion cancelled.".to_string())
            }
        }
    }

    /// Delete a subscription
    pub async fn delete_subscription(&self, id: Uuid) -> Result<String, AppError> {
        // check if the subscription exists
        let subscription = match self.storage.get_subscription(&id).await? {
            Some(sub) => sub,
            None => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Subscription with ID {} not found",
                    id
                )));
            }
        };

        // get the number of updates
        let updates = self.storage.get_updates_for_subscription(&id).await?;
        let updates_count = updates.len();

        if updates_count > 0 {
            // if there are related updates, prompt the user and request confirmation
            println!("This subscription has {} related updates.", updates_count);
            println!("Deleting this subscription will also delete these updates.");
            print!("Do you want to proceed? (y/N): ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() == "y" {
                // user confirmed, execute cascade deletion
                let deleted_updates = self.storage.cascade_delete_subscription(&id).await?;

                let display_name = subscription.name.clone();

                return Ok(format!(
                    "Subscription {} deleted along with {} updates.",
                    Color::Green.paint(&display_name),
                    Color::Green.paint(deleted_updates.to_string())
                ));
            } else {
                return Ok("Deletion cancelled.".to_string());
            }
        }

        // if there are no related updates, delete the subscription directly
        self.storage.delete_subscription(&id).await?;

        // Source ID usually includes the type prefix, extract the meaningful part
        let display_id = match subscription.source_id.split_once(':') {
            Some((_, id)) => id.to_string(),
            None => subscription.source_id.clone(),
        };

        Ok(format!("Subscription deleted: {}", display_id))
    }
}
