use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::atomic::{AtomicI32, Ordering};

use super::error::StorageError;
use super::{RepositoryStorage, Storage};
use crate::models::{Repository, SourceType};

/// In-memory implementation of Storage using DashMap
#[derive(Debug)]
pub struct MemoryStorage {
    repositories: DashMap<i32, Repository>,
    source_id_gen: AtomicI32,
}

// 手动实现 Clone
impl Clone for MemoryStorage {
    fn clone(&self) -> Self {
        Self {
            repositories: self.repositories.clone(),
            source_id_gen: AtomicI32::new(self.source_id_gen.load(Ordering::SeqCst)),
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
            source_id_gen: AtomicI32::new(1),
        }
    }

    fn next_repository_id(&self) -> i32 {
        self.source_id_gen.fetch_add(1, Ordering::SeqCst)
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
    ) -> Result<Option<Repository>, StorageError> {
        let repo = self
            .repositories
            .iter()
            .find(|r| r.owner == owner && r.name == name)
            .map(|r| r.clone());
        Ok(repo)
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
            return Err(StorageError::NotFound(SourceType::GitHub, id));
        }

        // delete the repository
        self.repositories.remove(&id);
        Ok(())
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn generate_id(&self) -> Result<i32, StorageError> {
        Ok(self.next_repository_id())
    }
}
