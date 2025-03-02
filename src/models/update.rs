use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of GitHub event
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateEventType {
    /// New commit
    Commit,
    /// New pull request
    PullRequest,
    /// Pull request update
    PullRequestUpdate,
    /// New issue
    Issue,
    /// Issue update
    IssueUpdate,
    /// New release
    Release,
}

/// Represents an update event from a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    /// Unique identifier for the update
    pub id: Uuid,

    /// Repository this update is from
    pub repository_id: Uuid,

    /// Type of event
    pub event_type: UpdateEventType,

    /// Event title
    pub title: String,

    /// Event description or content
    pub description: Option<String>,

    /// URL to the event
    pub url: String,

    /// Author of the event
    pub author: Option<String>,

    /// When the event occurred
    pub event_date: DateTime<Utc>,

    /// When we fetched this event
    pub fetched_at: DateTime<Utc>,

    /// Additional data in JSON format
    pub additional_data: Option<String>,
}

impl Update {
    /// Create a new update
    pub fn new(
        repository_id: Uuid,
        event_type: UpdateEventType,
        title: String,
        description: Option<String>,
        url: String,
        author: Option<String>,
        event_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            repository_id,
            event_type,
            title,
            description,
            url,
            author,
            event_date,
            fetched_at: Utc::now(),
            additional_data: None,
        }
    }
}
