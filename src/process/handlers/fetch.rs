//! Fetch command handler

use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::command::FetchCommand;
use crate::models::source::SourceFactory;
use crate::models::{AppConfig, UpdateEventType};
use crate::process::ProcessError;
use crate::storage::Storage;

/// Handler for fetch commands
pub struct FetchHandler<'a, S: Storage> {
    storage: Arc<S>,
    source_factory: Arc<dyn SourceFactory>,
    config: &'a AppConfig,
}

impl<'a, S: Storage> FetchHandler<'a, S> {
    /// Create a new fetch handler
    pub fn new(
        storage: Arc<S>,
        source_factory: Arc<dyn SourceFactory>,
        config: &'a AppConfig,
    ) -> Self {
        Self {
            storage,
            source_factory,
            config,
        }
    }

    /// Handle the fetch command
    pub async fn handle(&self, command: FetchCommand) -> Result<String, ProcessError> {
        match command {
            FetchCommand::Updates {
                subscription_id,
                days,
            } => self.fetch_updates(subscription_id, days).await,
        }
    }

    /// Fetch updates for a subscription
    async fn fetch_updates(
        &self,
        subscription_id: Uuid,
        days: Option<u32>,
    ) -> Result<String, ProcessError> {
        // Get the subscription
        let subscription = match self.storage.get_subscription(&subscription_id).await? {
            Some(sub) => sub,
            None => {
                return Err(ProcessError::not_found(format!(
                    "Subscription with ID {} not found",
                    subscription_id
                )));
            }
        };

        // Calculate the start date
        let days = days.unwrap_or(self.config.default_fetch_days);
        let start_date = Utc::now() - Duration::days(days as i64);

        // Get the source
        let source = self
            .source_factory
            .create_source(subscription.source_config)
            .await
            .map_err(ProcessError::SourceError)?;

        // Fetch updates from the source
        let updates = source
            .fetch_updates(Some(start_date))
            .await
            .map_err(ProcessError::SourceError)?;

        // Save updates
        for update in &updates {
            self.storage.save_update(update.clone()).await?;
        }

        // Group and count updates by type
        let mut commit_count = 0;
        let mut issue_count = 0;
        let mut pr_count = 0;
        let mut release_count = 0;
        let mut other_count = 0;

        for update in &updates {
            match update.event_type {
                UpdateEventType::Commit => commit_count += 1,
                UpdateEventType::Issue => issue_count += 1,
                UpdateEventType::PullRequest => pr_count += 1,
                UpdateEventType::Release => release_count += 1,
                _ => other_count += 1,
            }
        }

        // Format result message
        let mut result = format!(
            "Fetched {} updates from the last {} days:\n",
            updates.len(),
            days
        );

        if commit_count > 0 {
            result.push_str(&format!("- Commits: {}\n", commit_count));
        }

        if issue_count > 0 {
            result.push_str(&format!("- Issues: {}\n", issue_count));
        }

        if pr_count > 0 {
            result.push_str(&format!("- Pull Requests: {}\n", pr_count));
        }

        if release_count > 0 {
            result.push_str(&format!("- Releases: {}\n", release_count));
        }

        if other_count > 0 {
            result.push_str(&format!("- Other: {}\n", other_count));
        }

        Ok(result)
    }
}
