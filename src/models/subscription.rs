use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Repository;

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

/// Subscription represents a user's subscription to a GitHub repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique identifier for the subscription
    pub id: Uuid,

    /// The repository being subscribed to
    pub repository: Repository,

    /// Tags for categorizing this subscription
    pub tags: Vec<String>,

    /// How often to check for updates
    pub update_frequency: UpdateFrequency,

    /// What types of updates to track
    pub update_types: Vec<UpdateType>,

    /// When the subscription was created
    pub created_at: DateTime<Utc>,

    /// When the subscription was last updated
    pub updated_at: DateTime<Utc>,

    /// When updates were last fetched
    pub last_fetched: Option<DateTime<Utc>>,
}

impl Subscription {
    /// Create a new subscription for a repository
    pub fn new(
        repository: Repository,
        tags: Vec<String>,
        update_frequency: UpdateFrequency,
        update_types: Vec<UpdateType>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            repository,
            tags,
            update_frequency,
            update_types,
            created_at: now,
            updated_at: now,
            last_fetched: None,
        }
    }

    /// Create a simple subscription with default settings
    pub fn simple(repository: Repository) -> Self {
        Self::new(
            repository,
            Vec::new(),
            UpdateFrequency::default(),
            vec![UpdateType::All],
        )
    }
}
