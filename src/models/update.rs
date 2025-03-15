use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::source::SourceType;

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
}

/// Represents an update event from any source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    /// Source type (GitHub, HackerNews, etc.)
    pub source_type: SourceType,

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

    /// Additional data in JSON format
    pub additional_data: Option<serde_json::Value>,
}

impl Update {
    /// Create a new update
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_type: SourceType,
        event_type: UpdateEventType,
        title: String,
        description: Option<String>,
        url: String,
        author: Option<String>,
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
            additional_data: None,
        }
    }

    /// Create a new update with additional data
    #[allow(clippy::too_many_arguments)]
    pub fn with_data(
        source_type: SourceType,
        event_type: UpdateEventType,
        title: String,
        description: Option<String>,
        url: String,
        author: Option<String>,
        event_date: DateTime<Utc>,
        additional_data: serde_json::Value,
    ) -> Self {
        Self {
            source_type,
            event_type,
            title,
            description,
            url,
            author,
            event_date,
            additional_data: Some(additional_data),
        }
    }
}
