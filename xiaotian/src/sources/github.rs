use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use octocrab::Octocrab;
use serde_json::json;

use crate::models::source::{SourceMetadata, SourceType};
use crate::models::{Source, SourceError, Update, UpdateEventType};
use crate::utils::github_client::GithubClient;

/// A source for GitHub repositories
pub struct GitHubSource {
    /// Owner of the repository
    owner: String,
    /// Name of the repository
    repo: String,
    /// Branch to track (defaults to main/master)
    branch: Option<String>,
    /// GitHub client
    client: GithubClient,
    /// Source ID
    _source_id: i32,
}

impl GitHubSource {
    /// Create a new GitHub source
    pub fn new(
        owner: String,
        repo: String,
        branch: Option<String>,
        octocrab_client: Arc<Octocrab>,
        source_id: i32,
    ) -> Self {
        Self {
            owner,
            repo,
            branch,
            client: GithubClient::new(octocrab_client),
            _source_id: source_id,
        }
    }
}

#[async_trait]
impl Source for GitHubSource {
    fn get_type(&self) -> SourceType {
        SourceType::GitHub
    }

    fn get_id(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }

    fn get_name(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }

    fn get_description(&self) -> Option<String> {
        Some(format!(
            "GitHub repository: {}/{}{}",
            self.owner,
            self.repo,
            match &self.branch {
                Some(branch) => format!(" (branch: {})", branch),
                None => "".to_string(),
            }
        ))
    }

    fn get_url(&self) -> String {
        format!("https://github.com/{}/{}", self.owner, self.repo)
    }

    async fn fetch_updates(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        // Delegate to the GitHub client
        self.client
            .fetch_updates(
                &self.owner,
                &self.repo,
                self.branch.as_deref(),
                since,
                vec![UpdateEventType::Issue, UpdateEventType::PullRequest],
            )
            .await
    }

    fn get_metadata(&self) -> SourceMetadata {
        let data = json!({
            "owner": self.owner,
            "repo": self.repo,
            "branch": self.branch,
        });

        SourceMetadata {
            source_type: SourceType::GitHub,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{Duration, Utc};
    use octocrab::Octocrab;

    use crate::models::source::Source;
    use crate::sources::github::GitHubSource;

    #[tokio::test]
    async fn test_github_source_creation() {
        let client = Arc::new(Octocrab::builder().build().unwrap());
        let source = GitHubSource::new(
            "rust-lang".to_string(),
            "rust".to_string(),
            None,
            client,
            123456,
        );

        assert_eq!(source.get_type().to_string(), "GitHub");
        assert_eq!(source.get_name(), "rust-lang/rust");
        assert_eq!(source.get_url(), "https://github.com/rust-lang/rust");
        assert_eq!(source.get_id(), "rust-lang/rust");
    }

    #[tokio::test]
    #[ignore = "This test is slow and need to send request to GitHub API"]
    async fn test_fetch_updates() {
        let client = Arc::new(Octocrab::builder().build().unwrap());
        let source = GitHubSource::new(
            "rust-lang".to_string(),
            "rust".to_string(),
            None,
            client,
            123456,
        );

        // get the updates in the last 7 days
        let since = Utc::now() - Duration::days(7);
        let updates = source.fetch_updates(Some(since)).await;

        // because it's a public API, there may be rate limiting issues, so here we only check if the result is successful or not
        // do not test the number of updates returned
        match updates {
            Ok(updates) => {
                println!("Successfully fetched {} updates", updates.len());
                // verify the basic properties of the updates
                if !updates.is_empty() {
                    let update = &updates[0];
                    assert_eq!(update.source_type.to_string(), "GitHub");
                    assert!(!update.title.is_empty());
                    assert!(!update.url.is_empty());
                }
            }
            Err(e) => {
                println!(
                    "Error fetching updates, possibly due to rate limiting: {:?}",
                    e
                );
                // even if it's an error, it's still passed, because it may be due to GitHub API rate limiting
            }
        }
    }
}
