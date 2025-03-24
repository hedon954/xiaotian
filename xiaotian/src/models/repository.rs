use serde::{Deserialize, Serialize};

/// Repository represents a GitHub repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repository {
    /// Unique identifier for the repository in our system
    pub id: i32,

    /// Repository owner (username or organization)
    pub owner: String,

    /// Repository name
    pub name: String,
}

impl Repository {
    /// Create a new repository with minimal information
    pub fn new(owner: String, name: String) -> Self {
        Self { id: 0, owner, name }
    }

    /// Get the full repository name in the format "owner/name"
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
