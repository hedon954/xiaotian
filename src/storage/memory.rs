use async_trait::async_trait;
use dashmap::DashMap;
use serde_json::Value;
use uuid::Uuid;

use super::error::StorageError;
use super::{RepositoryStorage, Storage, SubscriptionStorage, UpdateStorage};
use crate::models::{Repository, Subscription, Update, UpdateEventType};

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
            .map(|v| v.clone())
            .ok_or(StorageError::NotFound(*id))
    }

    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError> {
        Ok(self
            .repositories
            .iter()
            .map(|item| item.value().clone())
            .collect())
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
                StorageError::Other(format!(
                    "Repository not found with owner/name: {}/{}",
                    owner, name
                ))
            })
    }

    async fn save_repository(&self, repository: Repository) -> Result<Repository, StorageError> {
        let id = repository.id;
        self.repositories.insert(id, repository.clone());
        Ok(repository)
    }

    async fn delete_repository(&self, id: &Uuid) -> Result<(), StorageError> {
        let related_subs = self.find_related_subscriptions(id).await?;
        if !related_subs.is_empty() {
            return Err(StorageError::RelatedEntitiesExist(related_subs.len()));
        }

        if self.repositories.remove(id).is_some() {
            Ok(())
        } else {
            Err(StorageError::NotFound(*id))
        }
    }

    async fn find_related_subscriptions(
        &self,
        id: &Uuid,
    ) -> Result<Vec<Subscription>, StorageError> {
        let repo = self.get_repository(id).await?;
        let repo_source_id = format!("github:{}:{}", repo.owner, repo.name);

        let related_subs: Vec<Subscription> = self
            .subscriptions
            .iter()
            .filter(|s| s.source_id == repo_source_id)
            .map(|s| s.clone())
            .collect();

        Ok(related_subs)
    }

    async fn cascade_delete_repository(&self, id: &Uuid) -> Result<(usize, usize), StorageError> {
        let related_subs = self.find_related_subscriptions(id).await?;

        if self.repositories.remove(id).is_none() {
            return Err(StorageError::NotFound(*id));
        }

        let mut total_subs_deleted = 0;
        let mut total_updates_deleted = 0;

        for sub in related_subs {
            let updates_deleted = self.cascade_delete_subscription(&sub.id).await?;
            total_updates_deleted += updates_deleted;
            total_subs_deleted += 1;
        }

        Ok((total_subs_deleted, total_updates_deleted))
    }
}

#[async_trait]
impl SubscriptionStorage for MemoryStorage {
    async fn get_subscription(&self, id: &Uuid) -> Result<Option<Subscription>, StorageError> {
        Ok(self.subscriptions.get(id).map(|v| v.clone()))
    }

    async fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, StorageError> {
        Ok(self
            .subscriptions
            .iter()
            .map(|item| item.value().clone())
            .collect())
    }

    async fn get_subscriptions_by_tag(&self, tag: &str) -> Result<Vec<Subscription>, StorageError> {
        let subs: Vec<Subscription> = self
            .subscriptions
            .iter()
            .filter(|s| s.tags.contains(&tag.to_string()))
            .map(|s| s.clone())
            .collect();

        Ok(subs)
    }

    async fn get_subscription_by_repository(
        &self,
        repo_id: &Uuid,
    ) -> Result<Option<Subscription>, StorageError> {
        // First get the repository to extract owner/name
        let repo = match self.get_repository(repo_id).await {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        // Look for a subscription with matching source_id
        let repo_source_id = format!("github:{}:{}", repo.owner, repo.name);

        let sub = self
            .subscriptions
            .iter()
            .find(|s| s.source_id == repo_source_id)
            .map(|s| s.clone());

        Ok(sub)
    }

    async fn get_subscriptions_by_source(
        &self,
        source_type: &str,
        source_id: &str,
    ) -> Result<Vec<Subscription>, StorageError> {
        let full_source_id = format!("{}:{}", source_type, source_id);

        let subs: Vec<Subscription> = self
            .subscriptions
            .iter()
            .filter(|s| s.source_id == full_source_id)
            .map(|s| s.clone())
            .collect();

        Ok(subs)
    }

    async fn verify_source_exists(
        &self,
        subscription: &Subscription,
    ) -> Result<bool, StorageError> {
        if subscription.source_type != crate::models::SourceType::GitHub {
            return Ok(true);
        }

        let parts: Vec<&str> = subscription.source_id.split(':').collect();
        if parts.len() != 3 || parts[0] != "github" {
            return Err(StorageError::ValidationError(
                "Invalid GitHub source ID format".to_string(),
            ));
        }

        let owner = parts[1];
        let name = parts[2];

        match self.get_repository_by_name(owner, name).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn save_subscription(
        &self,
        subscription: Subscription,
    ) -> Result<Subscription, StorageError> {
        if !self.verify_source_exists(&subscription).await? {
            return Err(StorageError::ReferenceIntegrityError(format!(
                "Source does not exist for subscription: {}",
                subscription.source_id
            )));
        }

        let id = subscription.id;
        self.subscriptions.insert(id, subscription.clone());
        Ok(subscription)
    }

    async fn delete_subscription(&self, id: &Uuid) -> Result<(), StorageError> {
        if self.subscriptions.remove(id).is_some() {
            Ok(())
        } else {
            Err(StorageError::NotFound(*id))
        }
    }

    async fn cascade_delete_subscription(&self, id: &Uuid) -> Result<usize, StorageError> {
        if self.subscriptions.get(id).is_none() {
            return Err(StorageError::NotFound(*id));
        }

        let updates_deleted = self.delete_updates_for_subscription(id).await?;
        self.subscriptions.remove(id);
        Ok(updates_deleted)
    }
}

#[async_trait]
impl UpdateStorage for MemoryStorage {
    async fn get_update(&self, id: &Uuid) -> Result<Update, StorageError> {
        self.updates
            .get(id)
            .map(|v| v.clone())
            .ok_or(StorageError::NotFound(*id))
    }

    async fn get_all_updates(&self) -> Result<Vec<Update>, StorageError> {
        Ok(self
            .updates
            .iter()
            .map(|item| item.value().clone())
            .collect())
    }

    async fn get_updates_for_repository(
        &self,
        repo_id: &Uuid,
    ) -> Result<Vec<Update>, StorageError> {
        // First get the repository to extract owner/name
        let repo = self.get_repository(repo_id).await?;

        // Look for updates with matching source_id
        let repo_source_id = format!("github:{}:{}", repo.owner, repo.name);

        let updates: Vec<Update> = self
            .updates
            .iter()
            .filter(|u| u.source_id == repo_source_id)
            .map(|u| u.clone())
            .collect();

        Ok(updates)
    }

    async fn get_updates_for_subscription(
        &self,
        subscription_id: &Uuid,
    ) -> Result<Vec<Update>, StorageError> {
        // get the subscription
        let subscription = match self.get_subscription(subscription_id).await? {
            Some(sub) => sub,
            None => return Err(StorageError::NotFound(*subscription_id)),
        };

        // find the updates with matching source_id
        let updates: Vec<Update> = self
            .updates
            .iter()
            .filter(|u| u.source_id == subscription.source_id)
            .map(|u| u.clone())
            .collect();

        Ok(updates)
    }

    async fn save_update(&self, update: Update) -> Result<Update, StorageError> {
        if self
            .updates
            .iter()
            .any(|existing| self.is_duplicate_update(&existing, &update))
        {
            return Ok(update);
        }

        let id = update.id;
        self.updates.insert(id, update.clone());
        Ok(update)
    }

    async fn delete_update(&self, id: &Uuid) -> Result<(), StorageError> {
        if self.updates.remove(id).is_some() {
            Ok(())
        } else {
            Err(StorageError::NotFound(*id))
        }
    }

    async fn delete_updates_for_subscription(
        &self,
        subscription_id: &Uuid,
    ) -> Result<usize, StorageError> {
        // get the subscription
        let subscription = match self.get_subscription(subscription_id).await? {
            Some(sub) => sub,
            None => return Err(StorageError::NotFound(*subscription_id)),
        };

        // get the related updates
        let updates = self
            .updates
            .iter()
            .filter(|u| u.source_id == subscription.source_id)
            .map(|u| *u.key())
            .collect::<Vec<Uuid>>();

        // delete all the related updates
        let mut count = 0;
        for update_id in updates {
            if self.updates.remove(&update_id).is_some() {
                count += 1;
            }
        }

        Ok(count)
    }
}

impl Storage for MemoryStorage {}
