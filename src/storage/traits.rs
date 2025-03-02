use async_trait::async_trait;
use uuid::Uuid;

use super::error::StorageError;
use crate::models::{Repository, Subscription, Update};

/// Repository storage operations
#[async_trait]
pub trait RepositoryStorage {
    /// Get a repository by ID
    async fn get_repository(&self, id: &Uuid) -> Result<Repository, StorageError>;

    /// Get all repositories
    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError>;

    /// Get repository by owner and name
    async fn get_repository_by_name(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Repository, StorageError>;

    /// Save a repository
    async fn save_repository(&self, repository: Repository) -> Result<Repository, StorageError>;

    /// Delete a repository
    async fn delete_repository(&self, id: &Uuid) -> Result<(), StorageError>;
}

/// Subscription storage operations
#[async_trait]
pub trait SubscriptionStorage {
    /// Get a subscription by ID
    async fn get_subscription(&self, id: &Uuid) -> Result<Subscription, StorageError>;

    /// Get all subscriptions
    async fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, StorageError>;

    /// Get subscriptions by tag
    async fn get_subscriptions_by_tag(&self, tag: &str) -> Result<Vec<Subscription>, StorageError>;

    /// Get subscription by repository ID
    async fn get_subscription_by_repository(
        &self,
        repo_id: &Uuid,
    ) -> Result<Option<Subscription>, StorageError>;

    /// Save a subscription
    async fn save_subscription(
        &self,
        subscription: Subscription,
    ) -> Result<Subscription, StorageError>;

    /// Delete a subscription
    async fn delete_subscription(&self, id: &Uuid) -> Result<(), StorageError>;
}

/// Update storage operations
#[async_trait]
pub trait UpdateStorage {
    /// Get an update by ID
    async fn get_update(&self, id: &Uuid) -> Result<Update, StorageError>;

    /// Get all updates
    async fn get_all_updates(&self) -> Result<Vec<Update>, StorageError>;

    /// Get updates for a repository
    async fn get_updates_for_repository(&self, repo_id: &Uuid)
    -> Result<Vec<Update>, StorageError>;

    /// Save an update
    async fn save_update(&self, update: Update) -> Result<Update, StorageError>;

    /// Delete an update
    async fn delete_update(&self, id: &Uuid) -> Result<(), StorageError>;
}

/// Combined storage interface
#[async_trait]
pub trait Storage: RepositoryStorage + SubscriptionStorage + UpdateStorage {
    /// Clear all data
    async fn clear(&self) -> Result<(), StorageError>;
}
