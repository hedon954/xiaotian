use async_trait::async_trait;
use dashmap::DashMap;
use serde_json::Value;
use std::sync::atomic::{AtomicI32, Ordering};
use uuid::Uuid;

use super::error::StorageError;
use super::{RepositoryStorage, Storage, SubscriptionStorage, UpdateStorage};
use crate::models::{Repository, Subscription, Update, UpdateEventType};

/// In-memory implementation of Storage using DashMap
#[derive(Debug)]
pub struct MemoryStorage {
    repositories: DashMap<i32, Repository>,
    subscriptions: DashMap<i32, Subscription>,
    updates: DashMap<Uuid, Update>,
    next_repo_id: AtomicI32,
    next_subscription_id: AtomicI32,
}

// 手动实现 Clone
impl Clone for MemoryStorage {
    fn clone(&self) -> Self {
        Self {
            repositories: self.repositories.clone(),
            subscriptions: self.subscriptions.clone(),
            updates: self.updates.clone(),
            next_repo_id: AtomicI32::new(self.next_repo_id.load(Ordering::SeqCst)),
            next_subscription_id: AtomicI32::new(self.next_subscription_id.load(Ordering::SeqCst)),
        }
    }
}

// 手动实现 Default
impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStorage {
    /// Create a new empty in-memory storage
    pub fn new() -> Self {
        Self {
            repositories: DashMap::new(),
            subscriptions: DashMap::new(),
            updates: DashMap::new(),
            next_repo_id: AtomicI32::new(1),
            next_subscription_id: AtomicI32::new(1),
        }
    }

    /// Get next repository ID
    pub fn next_repository_id(&self) -> i32 {
        self.next_repo_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Get next subscription ID
    pub fn next_subscription_id(&self) -> i32 {
        self.next_subscription_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Check if two updates represent the same commit
    fn is_same_commit(existing_data: &Value, update_data: &Value) -> bool {
        if let (Some(existing_sha), Some(update_sha)) = (
            existing_data.get("sha").and_then(|v| v.as_str()),
            update_data.get("sha").and_then(|v| v.as_str()),
        ) {
            existing_sha == update_sha
        } else {
            false
        }
    }

    /// Check if two updates represent the same issue or pull request
    fn is_same_issue_or_pr(existing_data: &Value, update_data: &Value) -> bool {
        if let (Some(existing_number), Some(update_number)) = (
            existing_data.get("number").and_then(|v| v.as_u64()),
            update_data.get("number").and_then(|v| v.as_u64()),
        ) {
            existing_number == update_number
        } else {
            false
        }
    }

    /// Check if two updates represent the same release
    fn is_same_release(existing_data: &Value, update_data: &Value) -> bool {
        // First try to match by ID
        if let (Some(existing_id), Some(update_id)) = (
            existing_data.get("id").and_then(|v| v.as_u64()),
            update_data.get("id").and_then(|v| v.as_u64()),
        ) {
            return existing_id == update_id;
        }

        // If no ID available, try matching by tag name
        if let (Some(existing_tag), Some(update_tag)) = (
            existing_data.get("tag_name").and_then(|v| v.as_str()),
            update_data.get("tag_name").and_then(|v| v.as_str()),
        ) {
            return existing_tag == update_tag;
        }

        false
    }

    /// Check if an update is an issue or PR related event
    fn is_issue_or_pr_type(event_type: UpdateEventType) -> bool {
        matches!(
            event_type,
            UpdateEventType::Issue
                | UpdateEventType::IssueUpdate
                | UpdateEventType::PullRequest
                | UpdateEventType::PullRequestUpdate
        )
    }

    /// Check if two updates are duplicates
    fn is_duplicate_update(&self, existing: &Update, update: &Update) -> bool {
        // Basic condition: must have same source and event type
        if existing.source_id != update.source_id || existing.event_type != update.event_type {
            return false;
        }

        // Only proceed with comparison if both updates have additional data
        if let (Some(existing_data), Some(update_data)) =
            (&existing.additional_data, &update.additional_data)
        {
            // Apply appropriate comparison based on event type
            match existing.event_type {
                UpdateEventType::Commit => {
                    return Self::is_same_commit(existing_data, update_data);
                }
                event_type if Self::is_issue_or_pr_type(event_type) => {
                    return Self::is_same_issue_or_pr(existing_data, update_data);
                }
                UpdateEventType::Release => {
                    return Self::is_same_release(existing_data, update_data);
                }
                _ => {}
            }
        }

        // Fallback to comparing basic properties when additional data isn't available
        // or for event types not handled above
        existing.title == update.title && existing.event_date == update.event_date
    }
}

#[async_trait]
impl RepositoryStorage for MemoryStorage {
    async fn get_repository(&self, id: i32) -> Result<Option<Repository>, StorageError> {
        Ok(self.repositories.get(&id).map(|r| r.clone()))
    }

    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError> {
        let repos = self
            .repositories
            .iter()
            .map(|r| r.clone())
            .collect::<Vec<_>>();
        Ok(repos)
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
            .ok_or_else(|| {
                StorageError::NotFound("Repository".to_string(), format!("{}/{}", owner, name))
            })
    }

    async fn save_repository(
        &self,
        mut repository: Repository,
    ) -> Result<Repository, StorageError> {
        if repository.id == 0 {
            repository.id = self.next_repository_id();
        }
        self.repositories.insert(repository.id, repository.clone());
        Ok(repository)
    }

    async fn delete_repository(&self, id: i32) -> Result<(), StorageError> {
        if !self.repositories.contains_key(&id) {
            return Err(StorageError::NotFound(
                "Repository".to_string(),
                id.to_string(),
            ));
        }

        // delete all related subscriptions
        let subs = self.find_related_subscriptions(id).await?;
        for sub in &subs {
            self.cascade_delete_subscription(sub.id).await?;
        }

        // delete the repository
        self.repositories.remove(&id);
        Ok(())
    }

    async fn find_related_subscriptions(
        &self,
        repo_id: i32,
    ) -> Result<Vec<Subscription>, StorageError> {
        let repo = self.get_repository(repo_id).await?;
        if let Some(repo) = repo {
            let subs = self
                .subscriptions
                .iter()
                .filter(|s| s.source_id == repo.id)
                .map(|s| s.clone())
                .collect::<Vec<_>>();
            Ok(subs)
        } else {
            Err(StorageError::NotFound(
                "Repository".to_string(),
                repo_id.to_string(),
            ))
        }
    }

    async fn cascade_delete_repository(&self, id: i32) -> Result<(usize, usize), StorageError> {
        if !self.repositories.contains_key(&id) {
            return Err(StorageError::NotFound(
                "Repository".to_string(),
                id.to_string(),
            ));
        }

        // find all subscriptions for the repository
        let subs = self
            .subscriptions
            .iter()
            .filter(|s| s.source_id == id)
            .map(|s| s.clone())
            .collect::<Vec<_>>();

        // delete all updates for the subscriptions
        let mut total_updates_deleted = 0;
        for sub in &subs {
            let updates_deleted = self.cascade_delete_subscription(sub.id).await?;
            total_updates_deleted += updates_deleted;
        }

        // delete the repository
        self.repositories.remove(&id);

        Ok((subs.len(), total_updates_deleted))
    }
}

#[async_trait]
impl SubscriptionStorage for MemoryStorage {
    async fn get_subscription(&self, id: i32) -> Result<Option<Subscription>, StorageError> {
        Ok(self.subscriptions.get(&id).map(|s| s.clone()))
    }

    async fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, StorageError> {
        let subs = self
            .subscriptions
            .iter()
            .map(|s| s.clone())
            .collect::<Vec<_>>();
        Ok(subs)
    }

    async fn get_subscriptions_by_tag(&self, tag: &str) -> Result<Vec<Subscription>, StorageError> {
        let subs = self
            .subscriptions
            .iter()
            .filter(|s| s.tags.contains(&tag.to_string()))
            .map(|s| s.clone())
            .collect::<Vec<_>>();
        Ok(subs)
    }

    async fn get_subscription_by_repository(
        &self,
        repo_id: i32,
    ) -> Result<Option<Subscription>, StorageError> {
        let Some(repo) = self.get_repository(repo_id).await? else {
            return Ok(None);
        };

        let sub = self
            .subscriptions
            .iter()
            .find(|s| s.source_id == repo.id)
            .map(|s| s.clone());

        Ok(sub)
    }

    async fn get_subscriptions_by_source(
        &self,
        source_id: i32,
    ) -> Result<Vec<Subscription>, StorageError> {
        let subs = self
            .subscriptions
            .iter()
            .filter(|s| s.source_id == source_id)
            .map(|s| s.clone())
            .collect::<Vec<_>>();
        Ok(subs)
    }

    async fn save_subscription(
        &self,
        mut subscription: Subscription,
    ) -> Result<Subscription, StorageError> {
        if subscription.id == 0 {
            subscription.id = self.next_subscription_id();
        }

        self.subscriptions
            .insert(subscription.id, subscription.clone());
        Ok(subscription)
    }

    async fn delete_subscription(&self, id: i32) -> Result<bool, StorageError> {
        if self.subscriptions.remove(&id).is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn cascade_delete_subscription(&self, id: i32) -> Result<usize, StorageError> {
        if !self.subscriptions.contains_key(&id) {
            return Err(StorageError::NotFound(
                "Subscription".to_string(),
                id.to_string(),
            ));
        }

        // delete all updates for the subscription
        let updates_deleted = self.delete_updates_for_subscription(id).await?;

        // delete the subscription
        self.subscriptions.remove(&id);

        Ok(updates_deleted)
    }
}

#[async_trait]
impl UpdateStorage for MemoryStorage {
    async fn get_update(&self, id: &Uuid) -> Result<Update, StorageError> {
        self.updates
            .get(id)
            .map(|u| u.clone())
            .ok_or_else(|| StorageError::NotFound("Update".to_string(), id.to_string()))
    }

    async fn get_all_updates(&self) -> Result<Vec<Update>, StorageError> {
        let updates = self.updates.iter().map(|u| u.clone()).collect::<Vec<_>>();
        Ok(updates)
    }

    async fn get_updates_for_repository(&self, repo_id: i32) -> Result<Vec<Update>, StorageError> {
        let updates = self
            .updates
            .iter()
            .filter(|u| u.source_id == repo_id)
            .map(|u| u.clone())
            .collect::<Vec<_>>();
        Ok(updates)
    }

    async fn get_updates_for_subscription(
        &self,
        subscription_id: i32,
    ) -> Result<Vec<Update>, StorageError> {
        let sub = self
            .get_subscription(subscription_id)
            .await?
            .ok_or_else(|| {
                StorageError::NotFound("Subscription".to_string(), subscription_id.to_string())
            })?;

        let updates = self
            .updates
            .iter()
            .filter(|u| u.source_type == sub.source_type && u.source_id == sub.source_id)
            .map(|u| u.clone())
            .collect::<Vec<_>>();
        Ok(updates)
    }

    async fn save_update(&self, update: Update) -> Result<Update, StorageError> {
        // 检查重复
        for existing in self.updates.iter() {
            if self.is_duplicate_update(&existing, &update) {
                return Ok(existing.clone());
            }
        }

        let id = update.id;
        self.updates.insert(id, update.clone());
        Ok(update)
    }

    async fn delete_update(&self, id: &Uuid) -> Result<bool, StorageError> {
        if self.updates.remove(id).is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_updates_for_subscription(
        &self,
        subscription_id: i32,
    ) -> Result<usize, StorageError> {
        let sub = self
            .get_subscription(subscription_id)
            .await?
            .ok_or_else(|| {
                StorageError::NotFound("Subscription".to_string(), subscription_id.to_string())
            })?;

        let to_delete: Vec<Uuid> = self
            .updates
            .iter()
            .filter(|u| u.source_type == sub.source_type && u.source_id == sub.source_id)
            .map(|u| u.id)
            .collect();

        let mut count = 0;
        for id in to_delete {
            if let Ok(deleted) = self.delete_update(&id).await {
                if deleted {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

impl Storage for MemoryStorage {}
