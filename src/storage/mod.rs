pub mod error;
pub mod memory;

pub use error::StorageError;
pub use memory::MemoryStorage;

use async_trait::async_trait;
use uuid::Uuid;

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

    /// Find subscriptions related to a repository
    async fn find_related_subscriptions(
        &self,
        id: &Uuid,
    ) -> Result<Vec<Subscription>, StorageError>;

    /// Delete a repository and all related subscriptions and updates (cascade delete)
    async fn cascade_delete_repository(&self, id: &Uuid) -> Result<(usize, usize), StorageError>;
}

/// Subscription storage operations
#[async_trait]
pub trait SubscriptionStorage {
    /// Get a subscription by ID
    async fn get_subscription(&self, id: &Uuid) -> Result<Option<Subscription>, StorageError>;

    /// Get all subscriptions
    async fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, StorageError>;

    /// Get subscriptions by tag
    async fn get_subscriptions_by_tag(&self, tag: &str) -> Result<Vec<Subscription>, StorageError>;

    /// Get subscription by repository ID
    async fn get_subscription_by_repository(
        &self,
        repo_id: &Uuid,
    ) -> Result<Option<Subscription>, StorageError>;

    /// Get subscriptions by source type and ID
    async fn get_subscriptions_by_source(
        &self,
        source_type: &str,
        source_id: &str,
    ) -> Result<Vec<Subscription>, StorageError>;

    /// Verify source exists for a subscription
    async fn verify_source_exists(&self, subscription: &Subscription)
    -> Result<bool, StorageError>;

    /// Save a subscription
    async fn save_subscription(
        &self,
        subscription: Subscription,
    ) -> Result<Subscription, StorageError>;

    /// Delete a subscription
    async fn delete_subscription(&self, id: &Uuid) -> Result<(), StorageError>;

    /// Delete a subscription and all related updates (cascade delete)
    async fn cascade_delete_subscription(&self, id: &Uuid) -> Result<usize, StorageError>;
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

    /// Get updates for a subscription
    async fn get_updates_for_subscription(
        &self,
        subscription_id: &Uuid,
    ) -> Result<Vec<Update>, StorageError>;

    /// Save an update
    async fn save_update(&self, update: Update) -> Result<Update, StorageError>;

    /// Delete an update
    async fn delete_update(&self, id: &Uuid) -> Result<(), StorageError>;

    /// Delete all updates for a subscription
    async fn delete_updates_for_subscription(
        &self,
        subscription_id: &Uuid,
    ) -> Result<usize, StorageError>;
}

/// Combined storage interface
#[async_trait]
pub trait Storage:
    RepositoryStorage + SubscriptionStorage + UpdateStorage + Send + Sync + 'static
{
}
