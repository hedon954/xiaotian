use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Repository represents a GitHub repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repository {
    /// Unique identifier for the repository in our system
    pub id: Uuid,

    /// Repository owner (username or organization)
    pub owner: String,

    /// Repository name
    pub name: String,

    /// Full repository URL
    pub url: String,

    /// Repository description
    pub description: Option<String>,

    /// Default branch (usually main or master)
    pub default_branch: Option<String>,

    /// Number of stars
    pub stars: Option<u64>,

    /// Number of forks
    pub forks: Option<u64>,

    /// When the repository was created in GitHub
    pub created_at: Option<DateTime<Utc>>,

    /// When the repository was last updated in GitHub
    pub updated_at: Option<DateTime<Utc>>,

    /// When we last fetched this repository's information
    pub last_fetched: Option<DateTime<Utc>>,
}

impl Repository {
    /// Create a new repository with minimal information
    pub fn new(owner: String, name: String) -> Self {
        let url = format!("https://github.com/{}/{}", owner, name);

        Self {
            id: Uuid::new_v4(),
            owner,
            name,
            url,
            description: None,
            default_branch: None,
            stars: None,
            forks: None,
            created_at: None,
            updated_at: None,
            last_fetched: None,
        }
    }

    /// Get the full repository name in the format "owner/name"
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
