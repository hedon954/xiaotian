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
        }
    }

    /// Get the full repository name in the format "owner/name"
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
