pub mod error;
pub mod memory;

pub use error::StorageError;
pub use memory::MemoryStorage;

use async_trait::async_trait;

use crate::models::{HackerNews, Repository};

/// Repository storage operations
#[async_trait]
pub trait RepositoryStorage {
    /// Get a repository by ID
    async fn get_repository(&self, id: i32) -> Result<Option<Repository>, StorageError>;

    /// Get all repositories
    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError>;

    /// Get repository by owner and name
    async fn get_repository_by_name(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Option<Repository>, StorageError>;

    /// Save a repository
    async fn save_repository(&self, repository: Repository) -> Result<Repository, StorageError>;

    /// Delete a repository
    async fn delete_repository(&self, id: i32) -> Result<(), StorageError>;
}

/// HackerNews storage operations
#[async_trait]
pub trait HackerNewsStorage {
    /// Get a hacker news by ID
    async fn get_hacker_news(&self, id: i32) -> Result<Option<HackerNews>, StorageError>;

    /// Get all hacker news
    async fn get_all_hacker_news(&self) -> Result<Vec<HackerNews>, StorageError>;

    /// Save a hacker news
    async fn save_hacker_news(&self, hacker_news: HackerNews) -> Result<HackerNews, StorageError>;

    /// Delete a hacker news
    async fn delete_hacker_news(&self, id: i32) -> Result<(), StorageError>;
}

/// Combined storage interface
#[async_trait]
pub trait Storage: RepositoryStorage + HackerNewsStorage + Send + Sync + 'static {
    async fn generate_id(&self) -> Result<i32, StorageError>;
}
