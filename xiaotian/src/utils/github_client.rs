use std::sync::Arc;

use chrono::{DateTime, Utc};
use octocrab::Octocrab;
use serde_json::json;

use crate::models::{SourceError, Update, UpdateEventType, source::SourceType};

/// Client for interacting with the GitHub API
pub struct GithubClient {
    /// GitHub API client from octocrab
    client: Arc<Octocrab>,
}

impl GithubClient {
    /// Create a new GitHub client
    pub fn new(client: Arc<Octocrab>) -> Self {
        Self { client }
    }

    /// Create a new GitHub client with a token
    pub fn with_token(token: String) -> Result<Self, SourceError> {
        let client = Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(Self::map_error)?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Create a new GitHub client without a token (anonymous)
    pub fn anonymous() -> Result<Self, SourceError> {
        let client = Octocrab::builder().build().map_err(Self::map_error)?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Get a reference to the internal Octocrab client
    pub fn get_octocrab(&self) -> Arc<Octocrab> {
        Arc::clone(&self.client)
    }

    /// Convert GitHub API errors to SourceError
    fn map_error<E: std::fmt::Debug>(err: E) -> SourceError {
        SourceError::ApiError(format!("GitHub API error: {:?}", err))
    }

    /// Fetch commits from GitHub
    pub async fn fetch_commits(
        &self,
        owner: &str,
        repo: &str,
        branch: Option<&str>,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Start with list_commits() builder
        let repo_handler = self.client.repos(owner, repo);
        let mut commits_handler = repo_handler.list_commits();

        // Add since filter if provided
        if let Some(since_time) = since {
            commits_handler = commits_handler.since(since_time);
        }

        // If a branch is specified, filter by that branch
        if let Some(branch_name) = branch {
            commits_handler = commits_handler.sha(branch_name);
        }

        // Execute the API call
        let commits_page = commits_handler.send().await.map_err(Self::map_error)?;

        // Convert each commit to an Update
        for commit in commits_page.items {
            let sha = commit.sha.clone();
            let author_name = commit.author.as_ref().map(|a| a.login.clone());

            // Get commit info from the first page
            // NOTE: We can't get detailed commit info due to API limitations with octocrab
            // We'll work with the basic info for now
            let commit_msg = commit.commit.message.clone();
            let title = commit_msg.lines().next().unwrap_or("").to_string();
            let description = if commit_msg.lines().count() > 1 {
                Some(commit_msg.lines().skip(1).collect::<Vec<&str>>().join("\n"))
            } else {
                None
            };

            let commit_date = match commit.commit.committer {
                Some(committer) => committer.date,
                None => None,
            };
            let commit_url = format!("https://github.com/{}/{}/commit/{}", owner, repo, sha);

            // Store additional data as JSON
            let additional_data = json!({
                "sha": sha,
                // We don't have stats in this payload, but we could fetch them if needed
            });

            let update = Update::with_data(
                SourceType::GitHub,
                UpdateEventType::Commit,
                title,
                description,
                commit_url,
                author_name,
                commit_date.unwrap_or_else(Utc::now),
                additional_data,
            );

            updates.push(update);
        }

        Ok(updates)
    }

    /// Fetch issues from GitHub
    pub async fn fetch_issues(
        &self,
        owner: &str,
        repo: &str,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Build the issues query
        let issues = self.client.issues(owner, repo);
        let mut issues_handler = issues.list().state(octocrab::params::State::Closed);

        // Add since filter if provided
        if let Some(since_time) = since {
            issues_handler = issues_handler.since(since_time);
        }

        // Execute the API call
        let issues_page = issues_handler.send().await.map_err(Self::map_error)?;

        // Convert each issue to an Update
        for issue in issues_page.items {
            // Skip pull requests (they're handled separately)
            if issue.pull_request.is_some() {
                continue;
            }

            let issue_num = issue.number;
            let issue_url = format!("https://github.com/{}/{}/issues/{}", owner, repo, issue_num);

            // Store additional data as JSON
            let additional_data = json!({
                "issue_number": issue_num,
                "state": format!("{:?}", issue.state),
                "labels": issue.labels.iter().map(|l| &l.name).collect::<Vec<_>>(),
                "comments": issue.comments,
            });

            let update = Update::with_data(
                SourceType::GitHub,
                UpdateEventType::Issue,
                issue.title,
                issue.body,
                issue_url,
                Some(issue.user.login),
                issue.updated_at,
                additional_data,
            );

            updates.push(update);
        }

        Ok(updates)
    }

    /// Fetch pull requests from GitHub
    pub async fn fetch_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Get pull requests
        let pulls_page = self
            .client
            .pulls(owner, repo)
            .list()
            .state(octocrab::params::State::Closed)
            .sort(octocrab::params::pulls::Sort::Updated)
            .direction(octocrab::params::Direction::Descending)
            .send()
            .await
            .map_err(Self::map_error)?;

        // Filter by date if needed
        let pulls = if let Some(since_time) = since {
            pulls_page
                .items
                .into_iter()
                .filter(|pr| pr.updated_at.unwrap_or_else(Utc::now) >= since_time)
                .collect::<Vec<_>>()
        } else {
            pulls_page.items
        };

        // Convert each PR to an Update
        for pr in pulls {
            let pr_num = pr.number;
            let pr_url = format!("https://github.com/{}/{}/pull/{}", owner, repo, pr_num);

            // Store additional data as JSON
            let additional_data = json!({
                "pr_number": pr_num,
                "state": format!("{:?}", pr.state.unwrap_or(octocrab::models::IssueState::Open)),
                "draft": pr.draft,
                "mergeable": pr.mergeable,
                "merged": pr.merged,
                "comments": pr.comments,
                "additions": pr.additions,
                "deletions": pr.deletions,
                "changed_files": pr.changed_files,
            });

            let update = Update::with_data(
                SourceType::GitHub,
                UpdateEventType::PullRequest,
                pr.title.unwrap_or_else(|| format!("PR #{}", pr_num)),
                pr.body,
                pr_url,
                pr.user.as_ref().map(|u| u.login.clone()),
                pr.updated_at.unwrap_or_else(Utc::now),
                additional_data,
            );

            updates.push(update);
        }

        Ok(updates)
    }

    /// Fetch releases from GitHub
    pub async fn fetch_releases(
        &self,
        owner: &str,
        repo: &str,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Get releases
        let releases_page = self
            .client
            .repos(owner, repo)
            .releases()
            .list()
            .send()
            .await
            .map_err(Self::map_error)?;

        // Filter by date if needed
        let releases = if let Some(since_time) = since {
            releases_page
                .items
                .into_iter()
                .filter(|release| release.published_at.unwrap_or_else(Utc::now) >= since_time)
                .collect::<Vec<_>>()
        } else {
            releases_page.items
        };

        // Convert each release to an Update
        for release in releases {
            let tag_name = release.tag_name;
            let release_url = release.html_url.to_string();

            // Store additional data as JSON
            let additional_data = json!({
                "tag_name": tag_name,
                "prerelease": release.prerelease,
                "draft": release.draft,
                "assets": release.assets.len(),
            });

            let update = Update::with_data(
                SourceType::GitHub,
                UpdateEventType::Release,
                release
                    .name
                    .unwrap_or_else(|| "Unnamed release".to_string()),
                release.body,
                release_url,
                release.author.map(|a| a.login),
                release.published_at.unwrap_or_else(Utc::now),
                additional_data,
            );

            updates.push(update);
        }

        Ok(updates)
    }

    pub async fn fetch_updates(
        &self,
        owner: &str,
        repo: &str,
        branch: Option<&str>,
        since: Option<DateTime<Utc>>,
        types: Vec<UpdateEventType>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut all_updates = Vec::new();

        if types.contains(&UpdateEventType::Commit) {
            let commits = self.fetch_commits(owner, repo, branch, since).await?;
            all_updates.extend(commits);
        }

        if types.contains(&UpdateEventType::Issue) {
            let issues = self.fetch_issues(owner, repo, since).await?;
            all_updates.extend(issues);
        }

        if types.contains(&UpdateEventType::PullRequest) {
            let prs = self.fetch_pull_requests(owner, repo, since).await?;
            all_updates.extend(prs);
        }

        if types.contains(&UpdateEventType::Release) {
            let releases = self.fetch_releases(owner, repo, since).await?;
            all_updates.extend(releases);
        }

        Ok(all_updates)
    }

    /// Fetch all update types from GitHub
    pub async fn fetch_all_updates(
        &self,
        owner: &str,
        repo: &str,
        branch: Option<&str>,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut all_updates = Vec::new();

        // Fetch commits
        let commits = self.fetch_commits(owner, repo, branch, since).await?;
        all_updates.extend(commits);

        // Fetch issues
        let issues = self.fetch_issues(owner, repo, since).await?;
        all_updates.extend(issues);

        // Fetch pull requests
        let prs = self.fetch_pull_requests(owner, repo, since).await?;
        all_updates.extend(prs);

        // Fetch releases
        let releases = self.fetch_releases(owner, repo, since).await?;
        all_updates.extend(releases);

        // Sort by date, newest first
        all_updates.sort_by(|a, b| b.event_date.cmp(&a.event_date));

        Ok(all_updates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = GithubClient::anonymous().unwrap();
        assert!(Arc::strong_count(&client.client) == 1);
    }

    // More tests would follow here
}
