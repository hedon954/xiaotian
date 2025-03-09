use async_trait::async_trait;
use dashmap::DashMap;
use serde_json::Value;
use uuid::Uuid;

use super::error::StorageError;
use super::{RepositoryStorage, Storage, SubscriptionStorage, UpdateStorage};
use crate::models::{Repository, SourceType, Subscription, Update, UpdateEventType};

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
    async fn get_repository(&self, id: &Uuid) -> Result<Repository, StorageError> {
        self.repositories
            .get(id)
            .map(|r| r.clone())
            .ok_or(StorageError::NotFound(*id))
    }

    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError> {
        let repositories = self.repositories.iter().map(|r| r.clone()).collect();
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
            .ok_or(StorageError::NotFound(Uuid::nil()))
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
        let subscriptions = self.subscriptions.iter().map(|s| s.clone()).collect();
        Ok(subscriptions)
    }

    async fn get_subscriptions_by_tag(&self, tag: &str) -> Result<Vec<Subscription>, StorageError> {
        let subscriptions = self
            .subscriptions
            .iter()
            .filter(|s| s.tags.contains(&tag.to_string()))
            .map(|s| s.clone())
            .collect();
        Ok(subscriptions)
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
            .ok_or(StorageError::NotFound(*id))
    }

    async fn get_all_updates(&self) -> Result<Vec<Update>, StorageError> {
        let updates = self.updates.iter().map(|u| u.clone()).collect();
        Ok(updates)
    }

    async fn get_updates_for_repository(
        &self,
        repo_id: &Uuid,
    ) -> Result<Vec<Update>, StorageError> {
        let repo_source_id = format!("github:{}", repo_id);

        let updates = self
            .updates
            .iter()
            .filter(|u| u.source_id == repo_source_id)
            .map(|u| u.clone())
            .collect();

        Ok(updates)
    }

    async fn save_update(&self, update: Update) -> Result<Update, StorageError> {
        // Check if this update is a duplicate of an existing one
        let is_duplicate = self
            .updates
            .iter()
            .any(|existing| self.is_duplicate_update(&existing, &update));

        // If it's a duplicate, return without saving
        if is_duplicate {
            return Ok(update);
        }

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
