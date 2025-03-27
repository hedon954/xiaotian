//! Fetch command handler

use std::sync::Arc;

use chrono::{Duration, Local, Utc};
use enum_dispatch::enum_dispatch;
use reedline_repl_rs::yansi::Paint;

use crate::Repository;
use crate::error::AppError;
use crate::models::source::SourceFactory;
use crate::models::{Fetcher, Source, SourceConfig, SourceType, SourceV2, UpdateEventType};
use crate::storage::Storage;

/// Handler for fetch commands
#[derive(Clone)]
pub struct FetchHandler<S: Storage> {
    storage: Arc<S>,
    source_factory: Arc<dyn SourceFactory>,
}

impl<S: Storage> FetchHandler<S> {
    /// Create a new fetch handler
    pub fn new(storage: Arc<S>, source_factory: Arc<dyn SourceFactory>) -> Self {
        Self {
            storage,
            source_factory,
        }
    }

    pub async fn fetch_updates_v2(&self, source: SourceV2, days: u32) -> Result<String, AppError> {
        let start_date = Local::now() - Duration::days(days as i64);
        let end_date = Local::now();
        source.fetch_updates(start_date, end_date).await
    }

    /// Fetch updates for a subscription
    pub async fn fetch_updates(&self, source_id: i32, days: u32) -> Result<String, AppError> {
        // Get the subscription
        let repository = match self.storage.get_repository(source_id).await? {
            Some(repo) => repo,
            None => {
                return Err(AppError::AnyError(anyhow::anyhow!(
                    "Repository with ID {} not found",
                    source_id
                )));
            }
        };

        // Calculate the start date
        let start_date = Utc::now() - Duration::days(days as i64);

        // Get the source
        let source = self
            .source_factory
            .create_source(
                SourceConfig {
                    source_type: SourceType::GitHub,
                    config: serde_json::json!({
                        "owner": repository.owner,
                        "repo": repository.name,
                    }),
                },
                source_id,
            )
            .await?;

        // Fetch updates from the source
        let updates = source
            .fetch_updates(Some(start_date))
            .await
            .map_err(AppError::SourceError)?;

        // Group and count updates by type
        let mut issue_count = 0;
        let mut pr_count = 0;
        let mut release_count = 0;

        // Format result message
        let mut result = format!(
            "Fetched {} updates from the last {} days:\n",
            updates.len(),
            days
        );
        let mut tmp_result = String::new();
        for update in updates {
            let r#type = match update.event_type {
                UpdateEventType::Issue => {
                    issue_count += 1;
                    "Issue".bright_green()
                }
                UpdateEventType::PullRequest => {
                    pr_count += 1;
                    "Pull Request".bright_yellow()
                }
                UpdateEventType::Release => {
                    release_count += 1;
                    "Release".bright_red()
                }
                _ => continue,
            };
            tmp_result.push_str(&format!(
                "-[{}] [{}] {}\n",
                r#type,
                update.event_date.format("%Y-%m-%d %H:%M:%S"),
                update.title,
            ));
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

        result.push_str(&tmp_result);
        Ok(result)
    }
}
