use async_trait::async_trait;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

use super::Fetcher;
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
    pub fn new(id: i32, owner: String, name: String) -> Self {
        Self { id, owner, name }
    }

    /// Get the full repository name in the format "owner/name"
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

#[async_trait]
impl Fetcher for Repository {
    async fn fetch_updates(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> Result<String, AppError> {
        todo!()
    }
}
