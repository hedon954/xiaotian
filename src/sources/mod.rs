pub mod github;

use async_trait::async_trait;

use crate::{
    error::AppError,
    models::{Source, SourceConfig, SourceFactory, SourceType, source::SourceConfigParser},
    utils::github_client::GithubClient,
};

pub use self::github::GitHubSource;

/// Factory for creating source instances
pub struct DefaultSourceFactory {
    github_client: GithubClient,
}

impl DefaultSourceFactory {
    /// Create a new source factory
    pub fn new(github_token: Option<String>) -> Result<Self, AppError> {
        let github_client = if let Some(token) = github_token {
            GithubClient::with_token(token).map_err(|e| {
                AppError::AnyError(anyhow::anyhow!("Failed to initialize GitHub client: {}", e))
            })?
        } else {
            GithubClient::anonymous().map_err(|e| {
                AppError::AnyError(anyhow::anyhow!("Failed to initialize GitHub client: {}", e))
            })?
        };

        Ok(Self { github_client })
    }
}

#[async_trait]
impl SourceFactory for DefaultSourceFactory {
    async fn create_source(
        &self,
        config: SourceConfig,
        source_id: i32,
    ) -> Result<Box<dyn Source>, AppError> {
        match config.source_type {
            SourceType::GitHub => {
                let github_config = config.parse_github_config()?;
                let source = github::GitHubSource::new(
                    github_config.owner,
                    github_config.repo,
                    github_config.branch,
                    self.github_client.get_octocrab(),
                    source_id,
                );
                Ok(Box::new(source))
            }
        }
    }
}
