pub mod github;

use std::sync::Arc;

use async_trait::async_trait;
use octocrab::Octocrab;

use crate::{
    error::AppError,
    models::{Source, SourceConfig, SourceFactory, SourceType, source::SourceConfigParser},
};

pub use self::github::GitHubSource;

/// Factory for creating source instances
pub struct DefaultSourceFactory {
    github_client: Arc<Octocrab>,
}

impl DefaultSourceFactory {
    /// Create a new source factory
    pub fn new(github_token: Option<String>) -> Result<Self, AppError> {
        let github_client = if let Some(token) = github_token {
            Octocrab::builder()
                .personal_token(token)
                .build()
                .map_err(|e| {
                    AppError::AnyError(anyhow::anyhow!("Failed to initialize GitHub client: {}", e))
                })?
        } else {
            Octocrab::builder().build().map_err(|e| {
                AppError::AnyError(anyhow::anyhow!("Failed to initialize GitHub client: {}", e))
            })?
        };

        Ok(Self {
            github_client: Arc::new(github_client),
        })
    }
}

#[async_trait]
impl SourceFactory for DefaultSourceFactory {
    async fn create_source(&self, config: SourceConfig) -> Result<Box<dyn Source>, AppError> {
        match config.source_type {
            SourceType::GitHub => {
                let github_config = config.parse_github_config()?;
                let source = github::GitHubSource::new(
                    github_config.owner,
                    github_config.repo,
                    github_config.branch,
                    Arc::clone(&self.github_client),
                );
                Ok(Box::new(source))
            }
        }
    }
}
