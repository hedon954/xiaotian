use serde::{Deserialize, Serialize};

use super::source::{SourceConfig, SourceType};

/// Frequency of update checks
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateFrequency {
    /// Check for updates daily
    Daily,
    /// Check for updates weekly
    Weekly,
    /// Check for updates manually only
    Manual,
}

impl Default for UpdateFrequency {
    fn default() -> Self {
        Self::Daily
    }
}

/// Types of updates to track
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateType {
    /// Track commits
    Commits,
    /// Track pull requests
    PullRequests,
    /// Track issues
    Issues,
    /// Track releases
    Releases,
    /// Track all update types
    All,
}

/// Subscription represents a user's subscription to any content source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique identifier for the subscription
    pub id: i32,

    /// Type of source (GitHub, HackerNews, etc.)
    pub source_type: SourceType,

    /// Unique identifier for the source
    pub source_id: i32,

    /// Source configuration
    pub source_config: SourceConfig,

    /// Display name for this subscription
    pub name: String,

    /// Tags for categorizing this subscription
    pub tags: Vec<String>,

    /// What types of updates to track
    pub update_types: Vec<UpdateType>,
}

impl Subscription {
    /// Create a new generic subscription
    pub fn new(
        name: String,
        source_type: SourceType,
        source_id: i32,
        source_config: SourceConfig,
        tags: Vec<String>,
        update_types: Vec<UpdateType>,
    ) -> Self {
        Self {
            id: 0,
            source_type,
            source_id,
            source_config,
            name,
            tags,
            update_types,
        }
    }

    /// Create a GitHub repository subscription
    pub fn github_repo(
        owner: String,
        repo: String,
        repo_id: i32,
        tags: Vec<String>,
        update_types: Vec<UpdateType>,
    ) -> Self {
        let name = format!("{}/{}", owner, repo);

        let source_config = SourceConfig {
            source_type: SourceType::GitHub,
            config: serde_json::json!({
                "owner": owner,
                "repo": repo,
                "track_types": update_types
                    .iter()
                    .map(|t| format!("{:?}", t).to_lowercase())
                    .collect::<Vec<String>>(),
            }),
        };
        Self::new(
            name,
            SourceType::GitHub,
            repo_id,
            source_config,
            tags,
            update_types,
        )
    }

    /// Create a simple GitHub subscription with default settings
    pub fn simple_github(owner: String, repo: String, repo_id: i32) -> Self {
        Self::github_repo(owner, repo, repo_id, Vec::new(), vec![UpdateType::All])
    }
}
