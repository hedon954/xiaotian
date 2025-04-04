use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::SourceType;

/// Type of update event
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum UpdateEventType {
    /// New commit
    Commit,
    /// New pull request
    PullRequest,
    /// New issue
    Issue,
    /// New release
    Release,
    /// New story (HackerNews)
    Story,
    /// New poll (HackerNews)
    Poll,
    /// New comment (HackerNews)
    Comment,
    /// New job posting (HackerNews)
    JobPosting,
    /// New question (like Ask HN)
    Question,
    /// New project showcase (like Show HN)
    Project,
}

/// Represents an update event from any source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    /// Source type
    pub source_type: SourceType,

    /// Type of event
    pub event_type: UpdateEventType,

    /// Event title
    pub title: String,

    /// Event description or content
    pub description: String,

    /// URL to the event
    pub url: String,

    /// Author of the event
    pub author: String,

    /// When the event occurred
    pub event_date: DateTime<Utc>,

    /// When the event was fetched
    pub fetched_at: DateTime<Utc>,

    /// Additional data in string format
    pub additional_data: Option<String>,
}

impl Update {
    /// Create a new update
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_type: SourceType,
        event_type: UpdateEventType,
        title: String,
        description: String,
        url: String,
        author: String,
        event_date: DateTime<Utc>,
    ) -> Self {
        Self {
            source_type,
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

    /// Create a new update with additional data
    #[allow(clippy::too_many_arguments)]
    pub fn with_data(
        source_type: SourceType,
        event_type: UpdateEventType,
        title: String,
        description: String,
        url: String,
        author: String,
        event_date: DateTime<Utc>,
        additional_data: String,
    ) -> Self {
        Self {
            source_type,
            event_type,
            title,
            description,
            url,
            author,
            event_date,
            fetched_at: Utc::now(),
            additional_data: Some(additional_data),
        }
    }
}
