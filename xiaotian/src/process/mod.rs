//! Command handlers module

pub mod schedule;

use std::sync::Arc;

use crate::{Repository, error::AppError, models::SourceType, storage::Storage};

use self::schedule::ScheduleHandler;

/// Processor for handling commands
#[derive(Clone)]
pub struct Processor<S: Storage> {
    storage: Arc<S>,
    pub schedule_handler: ScheduleHandler<S>,
}

impl<S: Storage> Processor<S> {
    /// Create a new processor
    pub fn new(storage: Arc<S>) -> Self {
        Self {
            storage: storage.clone(),
            schedule_handler: ScheduleHandler::new(storage.clone(), None),
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

    pub async fn run_all(&self, model: String, to_emails: Vec<String>) -> Result<(), AppError> {
        self.run_all_github(model.clone(), to_emails.clone())
            .await?;
        Ok(())
    }

    async fn run_all_github(&self, model: String, to_emails: Vec<String>) -> Result<(), AppError> {
        let repos = self.storage.get_all_repositories().await?;
        for repo in repos {
            self.schedule_handler
                .run_single(
                    SourceType::GitHub,
                    repo.id,
                    model.clone(),
                    to_emails.clone(),
                )
                .await?;
        }
        Ok(())
    }
}
