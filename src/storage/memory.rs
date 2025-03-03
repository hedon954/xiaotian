use async_trait::async_trait;
use dashmap::DashMap;
use uuid::Uuid;

use super::error::StorageError;
use super::{RepositoryStorage, Storage, SubscriptionStorage, UpdateStorage};
use crate::models::{Repository, SourceType, Subscription, Update};

/// In-memory implementation of Storage using DashMap
#[derive(Debug, Clone, Default)]
pub struct MemoryStorage {
    repositories: DashMap<Uuid, Repository>,
    subscriptions: DashMap<Uuid, Subscription>,
    updates: DashMap<Uuid, Update>,
}

impl MemoryStorage {
    /// Create a new empty in-memory storage
    pub fn new() -> Self {
        Self {
            repositories: DashMap::new(),
            subscriptions: DashMap::new(),
            updates: DashMap::new(),
        }
    }
}

#[async_trait]
impl RepositoryStorage for MemoryStorage {
    async fn get_repository(&self, id: &Uuid) -> Result<Repository, StorageError> {
        self.repositories
            .get(id)
            .map(|r| r.clone())
            .ok_or_else(|| StorageError::NotFound(*id))
    }

    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError> {
        let repositories: Vec<Repository> = self.repositories.iter().map(|r| r.clone()).collect();
        Ok(repositories)
    }

    async fn get_repository_by_name(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Repository, StorageError> {
        self.repositories
            .iter()
            .find(|r| r.owner == owner && r.name == name)
            .map(|r| r.clone())
            .ok_or_else(|| StorageError::Other(format!("Repository not found: {}/{}", owner, name)))
    }

    async fn save_repository(&self, repository: Repository) -> Result<Repository, StorageError> {
        let id = repository.id;
        self.repositories.insert(id, repository.clone());
        Ok(repository)
    }

    async fn delete_repository(&self, id: &Uuid) -> Result<(), StorageError> {
        if self.repositories.remove(id).is_none() {
            return Err(StorageError::NotFound(*id));
        }
        Ok(())
    }
}

#[async_trait]
impl SubscriptionStorage for MemoryStorage {
    async fn get_subscription(&self, id: &Uuid) -> Result<Option<Subscription>, StorageError> {
        Ok(self.subscriptions.get(id).map(|s| s.clone()))
    }

    async fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, StorageError> {
        let subscriptions: Vec<Subscription> =
            self.subscriptions.iter().map(|s| s.clone()).collect();
        Ok(subscriptions)
    }

    async fn get_subscriptions_by_tag(&self, tag: &str) -> Result<Vec<Subscription>, StorageError> {
        let filtered: Vec<Subscription> = self
            .subscriptions
            .iter()
            .filter(|s| s.tags.iter().any(|t| t == tag))
            .map(|s| s.clone())
            .collect();
        Ok(filtered)
    }

    async fn get_subscription_by_repository(
        &self,
        _repo_id: &Uuid,
    ) -> Result<Option<Subscription>, StorageError> {
        let subscription = self
            .subscriptions
            .iter()
            .find(|s| s.source_type == SourceType::GitHub && s.source_id.starts_with("github:"))
            .map(|s| s.clone());
        Ok(subscription)
    }

    async fn save_subscription(
        &self,
        subscription: Subscription,
    ) -> Result<Subscription, StorageError> {
        let id = subscription.id;
        self.subscriptions.insert(id, subscription.clone());
        Ok(subscription)
    }

    async fn delete_subscription(&self, id: &Uuid) -> Result<(), StorageError> {
        if self.subscriptions.remove(id).is_none() {
            return Err(StorageError::NotFound(*id));
        }
        Ok(())
    }
}

#[async_trait]
impl UpdateStorage for MemoryStorage {
    async fn get_update(&self, id: &Uuid) -> Result<Update, StorageError> {
        self.updates
            .get(id)
            .map(|u| u.clone())
            .ok_or_else(|| StorageError::NotFound(*id))
    }

    async fn get_all_updates(&self) -> Result<Vec<Update>, StorageError> {
        let updates: Vec<Update> = self.updates.iter().map(|u| u.clone()).collect();
        Ok(updates)
    }

    async fn get_updates_for_repository(
        &self,
        repo_id: &Uuid,
    ) -> Result<Vec<Update>, StorageError> {
        let _source_id = format!("github:{}", repo_id);

        let filtered: Vec<Update> = self
            .updates
            .iter()
            .filter(|u| {
                u.source_type == crate::models::SourceType::GitHub
                    && u.source_id.contains(&repo_id.to_string())
            })
            .map(|u| u.clone())
            .collect();
        Ok(filtered)
    }

    async fn save_update(&self, update: Update) -> Result<Update, StorageError> {
        let id = update.id;
        self.updates.insert(id, update.clone());
        Ok(update)
    }

    async fn delete_update(&self, id: &Uuid) -> Result<(), StorageError> {
        if self.updates.remove(id).is_none() {
            return Err(StorageError::NotFound(*id));
        }
        Ok(())
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn clear(&self) -> Result<(), StorageError> {
        self.repositories.clear();
        self.subscriptions.clear();
        self.updates.clear();
        Ok(())
    }
}
