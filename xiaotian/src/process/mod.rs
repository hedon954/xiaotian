//! Command handlers module

pub mod reporter;

use std::sync::Arc;

use crate::{
    Repository,
    error::AppError,
    models::{HackerNews, HackerNewsFeedType, SourceType},
    storage::Storage,
};

use self::reporter::Reporter;

/// Processor for handling commands
#[derive(Clone)]
pub struct Processor<S: Storage> {
    storage: Arc<S>,
    pub schedule_handler: Reporter<S>,
}

impl<S: Storage> Processor<S> {
    /// Create a new processor
    pub fn new(storage: Arc<S>) -> Self {
        Self {
            storage: storage.clone(),
            schedule_handler: Reporter::new(storage.clone(), None),
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

    pub async fn add_hacker_news(&self, feed_type: HackerNewsFeedType) -> Result<i32, AppError> {
        let id = self.storage.generate_id().await?;
        let hn = HackerNews::new(id, feed_type, 100, 20);
        self.storage.save_hacker_news(hn).await?;
        Ok(id)
    }

    pub async fn delete_hacker_news(&self, id: i32) -> Result<(), AppError> {
        self.storage.delete_hacker_news(id).await?;
        Ok(())
    }

    pub async fn list_hacker_news(&self) -> Result<Vec<HackerNews>, AppError> {
        let hns = self.storage.get_all_hacker_news().await?;
        Ok(hns)
    }

    pub async fn get_hacker_news(&self, id: i32) -> Result<Option<HackerNews>, AppError> {
        let hn = self.storage.get_hacker_news(id).await?;
        Ok(hn)
    }

    pub async fn run_all(&self, model: String, to_emails: Vec<String>) -> Result<(), AppError> {
        self.run_all_github(model.clone(), to_emails.clone())
            .await?;
        self.run_all_hacker_news(model.clone(), to_emails.clone())
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

    async fn run_all_hacker_news(
        &self,
        model: String,
        to_emails: Vec<String>,
    ) -> Result<(), AppError> {
        let hns = self.storage.get_all_hacker_news().await?;
        for hn in hns {
            self.schedule_handler
                .run_single(
                    SourceType::HackerNews,
                    hn.id,
                    model.clone(),
                    to_emails.clone(),
                )
                .await?;
        }
        Ok(())
    }
}
