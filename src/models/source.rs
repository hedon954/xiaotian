use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Update;

/// Represents the type of content source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    /// GitHub repository
    GitHub,
    // Future source types can be added here:
    // HackerNews,
    // Reddit,
    // WeChatOfficialAccount,
    // etc.
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::GitHub => write!(f, "GitHub"),
            // Add cases for future source types
        }
    }
}

/// Generic metadata structure for any source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetadata {
    /// Type of the source
    pub source_type: SourceType,

    /// Additional source-specific data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Error type for source operations
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Source not found: {0}")]
    NotFound(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Other error: {0}")]
    Other(String),
}

/// Trait representing a content source that can be subscribed to
#[async_trait]
pub trait Source: Send + Sync {
    /// Get the source type
    fn get_type(&self) -> SourceType;

    /// Get the unique identifier for this source
    fn get_id(&self) -> String;

    /// Get the display name of this source
    fn get_name(&self) -> String;

    /// Get a description for this source (if available)
    fn get_description(&self) -> Option<String>;

    /// Get the URL for this source
    fn get_url(&self) -> String;

    /// Fetch updates from this source since the specified time
    async fn fetch_updates(&self, since: Option<DateTime<Utc>>)
    -> Result<Vec<Update>, SourceError>;

    /// Get source-specific metadata
    fn get_metadata(&self) -> SourceMetadata;
}

/// Source configuration - used when creating a source from a subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    /// Type of source
    pub source_type: SourceType,

    /// Source-specific configuration
    #[serde(flatten)]
    pub config: serde_json::Value,
}

/// GitHub-specific source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubSourceConfig {
    /// Repository owner (user or organization)
    pub owner: String,

    /// Repository name
    pub repo: String,

    /// Branch to track (defaults to main/master if not specified)
    pub branch: Option<String>,

    /// Types of updates to track
    #[serde(default)]
    pub track_types: Vec<String>,
}

/// Helper functions to parse source configurations
pub trait SourceConfigParser {
    /// Parse the GitHub-specific configuration
    fn parse_github_config(&self) -> Result<GitHubSourceConfig, SourceError>;

    // Add methods for other source types in the future
}

impl SourceConfigParser for SourceConfig {
    fn parse_github_config(&self) -> Result<GitHubSourceConfig, SourceError> {
        if self.source_type != SourceType::GitHub {
            return Err(SourceError::ParseError(
                "Not a GitHub source configuration".to_string(),
            ));
        }

        serde_json::from_value(self.config.clone())
            .map_err(|e| SourceError::ParseError(format!("Failed to parse GitHub config: {}", e)))
    }
}

/// Factory trait for creating sources from configurations
#[async_trait]
pub trait SourceFactory: Send + Sync {
    /// Create a source instance from a configuration
    async fn create_source(&self, config: SourceConfig) -> Result<Box<dyn Source>, SourceError>;
}
