use std::sync::Arc;

use super::error::ReplError;
use crate::models::{Repository, Subscription};
use crate::storage::Storage;

/// Command handler for REPL
pub struct CommandHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> CommandHandler<S> {
    /// Create a new command handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Execute a command
    pub async fn execute(&self, command: &str, args: Vec<&str>) -> Result<String, ReplError> {
        match command {
            "help" => Ok(self.help()),
            "add" => self.add(args).await,
            "list" => self.list(args).await,
            "show" => self.show(args).await,
            "delete" => self.delete(args).await,
            "clear" => self.clear().await,
            _ => Err(ReplError::CommandNotFound(command.to_string())),
        }
    }

    /// Get help text
    fn help(&self) -> String {
        r#"
XiaoTian - GitHub Repository Tracker

Available commands:
  help                                Show this help message
  add repo <owner> <name>             Add a repository
  add subscription <owner> <name>     Subscribe to a repository
  list repos                          List all repositories
  list subscriptions                  List all subscriptions
  show repo <owner> <name>            Show repository details
  show subscription <id>              Show subscription details
  delete repo <owner> <name>          Delete a repository
  delete subscription <id>            Delete a subscription
  clear                               Clear all data
"#
        .to_string()
    }

    /// Add a repository or subscription
    async fn add(&self, args: Vec<&str>) -> Result<String, ReplError> {
        if args.is_empty() {
            return Err(ReplError::InvalidArguments(
                "Missing subcommand".to_string(),
            ));
        }

        match args[0] {
            "repo" => {
                if args.len() < 3 {
                    return Err(ReplError::InvalidArguments(
                        "Usage: add repo <owner> <name>".to_string(),
                    ));
                }

                let owner = args[1].to_string();
                let name = args[2].to_string();

                // Create a new repository
                let repository = Repository::new(owner.clone(), name.clone());

                // Save the repository
                self.storage.save_repository(repository).await?;

                Ok(format!("Repository added: {}/{}", owner, name))
            }
            "subscription" => {
                if args.len() < 3 {
                    return Err(ReplError::InvalidArguments(
                        "Usage: add subscription <owner> <name>".to_string(),
                    ));
                }

                let owner = args[1].to_string();
                let name = args[2].to_string();

                // Get the repository or create it if it doesn't exist
                let repository = match self.storage.get_repository_by_name(&owner, &name).await {
                    Ok(repo) => repo,
                    Err(_) => {
                        let repo = Repository::new(owner.clone(), name.clone());
                        self.storage.save_repository(repo).await?
                    }
                };

                // Create a new subscription
                let subscription = Subscription::simple(repository);

                // Save the subscription
                self.storage.save_subscription(subscription).await?;

                Ok(format!("Subscription added for: {}/{}", owner, name))
            }
            _ => Err(ReplError::InvalidArguments(format!(
                "Unknown subcommand: {}",
                args[0]
            ))),
        }
    }

    /// List repositories or subscriptions
    async fn list(&self, args: Vec<&str>) -> Result<String, ReplError> {
        if args.is_empty() {
            return Err(ReplError::InvalidArguments(
                "Missing subcommand".to_string(),
            ));
        }

        match args[0] {
            "repos" => {
                let repositories = self.storage.get_all_repositories().await?;

                if repositories.is_empty() {
                    return Ok("No repositories found".to_string());
                }

                let mut result = String::from("Repositories:\n");
                for repo in repositories {
                    result.push_str(&format!("- {}/{}\n", repo.owner, repo.name));
                }

                Ok(result)
            }
            "subscriptions" => {
                let subscriptions = self.storage.get_all_subscriptions().await?;

                if subscriptions.is_empty() {
                    return Ok("No subscriptions found".to_string());
                }

                let mut result = String::from("Subscriptions:\n");
                for sub in subscriptions {
                    result.push_str(&format!("- {} ({})\n", sub.repository.full_name(), sub.id));
                }

                Ok(result)
            }
            _ => Err(ReplError::InvalidArguments(format!(
                "Unknown subcommand: {}",
                args[0]
            ))),
        }
    }

    /// Show repository or subscription details
    async fn show(&self, args: Vec<&str>) -> Result<String, ReplError> {
        if args.is_empty() {
            return Err(ReplError::InvalidArguments(
                "Missing subcommand".to_string(),
            ));
        }

        match args[0] {
            "repo" => {
                if args.len() < 3 {
                    return Err(ReplError::InvalidArguments(
                        "Usage: show repo <owner> <name>".to_string(),
                    ));
                }

                let owner = args[1];
                let name = args[2];

                let repository = self.storage.get_repository_by_name(owner, name).await?;

                let mut result = String::new();
                result.push_str(&format!(
                    "Repository: {}/{}\n",
                    repository.owner, repository.name
                ));
                result.push_str(&format!("ID: {}\n", repository.id));
                result.push_str(&format!("URL: {}\n", repository.url));

                if let Some(desc) = &repository.description {
                    result.push_str(&format!("Description: {}\n", desc));
                }

                if let Some(stars) = repository.stars {
                    result.push_str(&format!("Stars: {}\n", stars));
                }

                if let Some(forks) = repository.forks {
                    result.push_str(&format!("Forks: {}\n", forks));
                }

                Ok(result)
            }
            "subscription" => {
                if args.len() < 2 {
                    return Err(ReplError::InvalidArguments(
                        "Usage: show subscription <id>".to_string(),
                    ));
                }

                let id_str = args[1];
                let id = id_str.parse::<uuid::Uuid>().map_err(|_| {
                    ReplError::InvalidArguments(format!("Invalid UUID: {}", id_str))
                })?;

                let subscription = self.storage.get_subscription(&id).await?;

                let mut result = String::new();
                result.push_str(&format!("Subscription: {}\n", subscription.id));
                result.push_str(&format!(
                    "Repository: {}/{}\n",
                    subscription.repository.owner, subscription.repository.name
                ));

                if !subscription.tags.is_empty() {
                    result.push_str(&format!("Tags: {}\n", subscription.tags.join(", ")));
                }

                result.push_str(&format!(
                    "Update frequency: {:?}\n",
                    subscription.update_frequency
                ));

                let update_types: Vec<String> = subscription
                    .update_types
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect();
                result.push_str(&format!("Update types: {}\n", update_types.join(", ")));

                result.push_str(&format!("Created at: {}\n", subscription.created_at));

                Ok(result)
            }
            _ => Err(ReplError::InvalidArguments(format!(
                "Unknown subcommand: {}",
                args[0]
            ))),
        }
    }

    /// Delete a repository or subscription
    async fn delete(&self, args: Vec<&str>) -> Result<String, ReplError> {
        if args.is_empty() {
            return Err(ReplError::InvalidArguments(
                "Missing subcommand".to_string(),
            ));
        }

        match args[0] {
            "repo" => {
                if args.len() < 3 {
                    return Err(ReplError::InvalidArguments(
                        "Usage: delete repo <owner> <name>".to_string(),
                    ));
                }

                let owner = args[1];
                let name = args[2];

                let repository = self.storage.get_repository_by_name(owner, name).await?;
                let id = repository.id;

                self.storage.delete_repository(&id).await?;

                Ok(format!("Repository deleted: {}/{}", owner, name))
            }
            "subscription" => {
                if args.len() < 2 {
                    return Err(ReplError::InvalidArguments(
                        "Usage: delete subscription <id>".to_string(),
                    ));
                }

                let id_str = args[1];
                let id = id_str.parse::<uuid::Uuid>().map_err(|_| {
                    ReplError::InvalidArguments(format!("Invalid UUID: {}", id_str))
                })?;

                self.storage.delete_subscription(&id).await?;

                Ok(format!("Subscription deleted: {}", id))
            }
            _ => Err(ReplError::InvalidArguments(format!(
                "Unknown subcommand: {}",
                args[0]
            ))),
        }
    }

    /// Clear all data
    async fn clear(&self) -> Result<String, ReplError> {
        self.storage.clear().await?;

        Ok("All data cleared".to_string())
    }
}
