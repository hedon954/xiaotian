//! Command handlers module

pub mod fetch;
pub mod schedule;

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Local, Offset, Utc};
use enum_dispatch::enum_dispatch;

use crate::{Repository, error::AppError, models::source::SourceFactory, storage::Storage};

use self::{fetch::FetchHandler, schedule::ScheduleHandler};

/// Processor for handling commands
pub struct Processor<S: Storage> {
    storage: Arc<S>,
    pub fetch_handler: FetchHandler<S>,
    pub schedule_handler: ScheduleHandler<S>,
}

impl<S: Storage> Processor<S> {
    /// Create a new processor
    pub fn new(storage: Arc<S>, source_factory: Arc<dyn SourceFactory>) -> Self {
        Self {
            storage: storage.clone(),
            fetch_handler: FetchHandler::new(storage.clone(), source_factory.clone()),
            schedule_handler: ScheduleHandler::new(storage.clone(), source_factory, None),
        }
    }

    pub async fn add_repository(&self, owner: String, name: String) -> Result<i32, AppError> {
        let id = self.storage.generate_id().await?;
        let repo = Repository::new(id, owner, name);
        self.storage.save_repository(repo).await?;
        Ok(id)
    }

    pub async fn delete_repository(&self, id: i32) -> Result<(), AppError> {
        self.storage.delete_repository(id).await?;
        Ok(())
    }

    pub async fn list_repositories(&self) -> Result<Vec<Repository>, AppError> {
        let repos = self.storage.get_all_repositories().await?;
        Ok(repos)
    }

    pub async fn get_repository(&self, id: i32) -> Result<Option<Repository>, AppError> {
        let repo = self.storage.get_repository(id).await?;
        Ok(repo)
    }

    pub async fn get_repository_by_name(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Option<Repository>, AppError> {
        let repo = self.storage.get_repository_by_name(owner, name).await?;
        Ok(repo)
    }
}
