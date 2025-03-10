//! Show command handler

use std::sync::Arc;

use colored::Colorize;

use crate::error::AppError;
use crate::models::{Update, UpdateEventType};
use crate::storage::Storage;

/// Handler for show commands
#[derive(Clone)]
pub struct ShowHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> ShowHandler<S> {
    /// Create a new show handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Show repository details
    pub async fn show_repository(&self, owner: String, name: String) -> Result<String, AppError> {
        let repo = match self.storage.get_repository_by_name(&owner, &name).await {
            Ok(repo) => repo,
            Err(_) => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Repository '{}/{}' not found",
                    owner,
                    name
                )));
            }
        };

        let mut result = format!("Repository: {}/{}\n", repo.owner, repo.name);
        result.push_str(&format!("ID: {}\n", repo.id.to_string().bright_blue()));
        result.push_str(&format!("URL: {}\n", repo.url));

        Ok(result)
    }

    /// Show subscription details
    pub async fn show_subscription(&self, id: i32) -> Result<String, AppError> {
        let subscription = match self.storage.get_subscription(id).await? {
            Some(sub) => sub,
            None => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Subscription with ID {} not found",
                    id
                )));
            }
        };

        let mut result = format!("Subscription: {}\n", subscription.name);
        result.push_str(&format!(
            "ID: {}\n",
            subscription.id.to_string().bright_blue()
        ));
        result.push_str(&format!("Source Type: {:?}\n", subscription.source_type));

        result.push_str("Update Types: ");
        if subscription.update_types.is_empty() {
            result.push_str("None\n");
        } else {
            result.push_str(&format!("{:?}\n", subscription.update_types));
        }

        result.push_str("Tags: ");
        if subscription.tags.is_empty() {
            result.push_str("None\n");
        } else {
            result.push_str(&format!("{:?}\n", subscription.tags));
        }

        Ok(result)
    }

    /// Show updates for a subscription
    pub async fn show_updates(
        &self,
        subscription_id: i32,
        limit: usize,
    ) -> Result<String, AppError> {
        // Check if subscription exists
        let subscription = match self.storage.get_subscription(subscription_id).await? {
            Some(sub) => sub,
            None => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Subscription with ID {} not found",
                    subscription_id
                )));
            }
        };

        // Get updates for repository
        let updates = self
            .storage
            .get_updates_for_repository(subscription.source_id)
            .await?;

        if updates.is_empty() {
            return Ok(format!(
                "No updates found for subscription {}.",
                subscription_id
            ));
        }

        // Format updates
        self.format_updates(updates, limit)
    }

    /// Format updates for display
    fn format_updates(&self, mut updates: Vec<Update>, limit: usize) -> Result<String, AppError> {
        // Sort updates by date (newest first)
        updates.sort_by(|a, b| b.event_date.cmp(&a.event_date));

        // Limit the number of updates
        let limited_updates = if updates.len() > limit {
            updates.truncate(limit);
            format!(
                "Found {} updates (showing the latest {}):\n\n",
                updates.len(),
                limit
            )
        } else {
            format!("Found {} updates:\n\n", updates.len())
        };

        let mut result = limited_updates;

        // Group updates by type
        let mut commits = Vec::new();
        let mut issues = Vec::new();
        let mut pull_requests = Vec::new();
        let mut releases = Vec::new();
        let mut others = Vec::new();

        for update in updates {
            match update.event_type {
                UpdateEventType::Commit => commits.push(update),
                UpdateEventType::Issue => issues.push(update),
                UpdateEventType::PullRequest => pull_requests.push(update),
                UpdateEventType::Release => releases.push(update),
                _ => others.push(update),
            }
        }

        // Add commits
        if !commits.is_empty() {
            result.push_str(&format!("Commits ({}):\n", commits.len()));
            for commit in commits {
                result.push_str(&format!(
                    "  - [{}] {}\n",
                    commit.event_date.format("%Y-%m-%d"),
                    commit.title
                ));
            }
            result.push('\n');
        }

        // Add issues
        if !issues.is_empty() {
            result.push_str(&format!("Issues ({}):\n", issues.len()));
            for issue in issues {
                result.push_str(&format!(
                    "  - [{}] {}\n",
                    issue.event_date.format("%Y-%m-%d"),
                    issue.title
                ));
            }
            result.push('\n');
        }

        // Add pull requests
        if !pull_requests.is_empty() {
            result.push_str(&format!("Pull Requests ({}):\n", pull_requests.len()));
            for pr in pull_requests {
                result.push_str(&format!(
                    "  - [{}] {}\n",
                    pr.event_date.format("%Y-%m-%d"),
                    pr.title
                ));
            }
            result.push('\n');
        }

        // Add releases
        if !releases.is_empty() {
            result.push_str(&format!("Releases ({}):\n", releases.len()));
            for release in releases {
                result.push_str(&format!(
                    "  - [{}] {}\n",
                    release.event_date.format("%Y-%m-%d"),
                    release.title
                ));
            }
            result.push('\n');
        }

        // Add other types
        if !others.is_empty() {
            result.push_str(&format!("Other Updates ({}):\n", others.len()));
            for other in others {
                result.push_str(&format!(
                    "  - [{}] {}\n",
                    other.event_date.format("%Y-%m-%d"),
                    other.title
                ));
            }
        }

        Ok(result)
    }
}
