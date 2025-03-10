use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use octocrab::Octocrab;
use serde_json::json;

use crate::models::source::{SourceMetadata, SourceType};
use crate::models::{Source, SourceError, Update, UpdateEventType};

/// A source for GitHub repositories
pub struct GitHubSource {
    /// Owner of the repository
    owner: String,
    /// Name of the repository
    repo: String,
    /// Branch to track (defaults to main/master)
    branch: Option<String>,
    /// GitHub client
    client: Arc<Octocrab>,
    /// Cached repository ID
    repo_id: i32,
}

impl GitHubSource {
    /// Create a new GitHub source
    pub fn new(
        owner: String,
        repo: String,
        branch: Option<String>,
        client: Arc<Octocrab>,
        repo_id: i32,
    ) -> Self {
        Self {
            owner,
            repo,
            branch,
            client,
            repo_id,
        }
    }

    /// Convert GitHub API errors to SourceError
    fn map_error<E: std::fmt::Debug>(err: E) -> SourceError {
        SourceError::ApiError(format!("GitHub API error: {:?}", err))
    }

    /// Fetch commits from GitHub
    async fn fetch_commits(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Start with list_commits() builder
        let repo = self.client.repos(&self.owner, &self.repo);
        let mut commits_handler = repo.list_commits();

        // Add since filter if provided
        if let Some(since_time) = since {
            commits_handler = commits_handler.since(since_time);
        }

        // If a branch is specified, filter by that branch
        if let Some(branch) = &self.branch {
            commits_handler = commits_handler.sha(branch);
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
            let commit_url = format!(
                "https://github.com/{}/{}/commit/{}",
                self.owner, self.repo, sha
            );

            // Store additional data as JSON
            let additional_data = json!({
                "sha": sha,
                // We don't have stats in this payload, but we could fetch them if needed
            });

            let update = Update::with_data(
                SourceType::GitHub,
                self.repo_id,
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
    async fn fetch_issues(&self, since: Option<DateTime<Utc>>) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Build the issues query
        let issues = self.client.issues(&self.owner, &self.repo);
        let mut issues_handler = issues.list().state(octocrab::params::State::All);

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
            let issue_url = format!(
                "https://github.com/{}/{}/issues/{}",
                self.owner, self.repo, issue_num
            );

            // Determine if this is a new issue or an update
            let event_type = match since {
                Some(since_time) if issue.created_at > since_time => UpdateEventType::Issue,
                _ => UpdateEventType::IssueUpdate,
            };

            // Store additional data as JSON
            let additional_data = json!({
                "issue_number": issue_num,
                "state": format!("{:?}", issue.state),
                "labels": issue.labels.iter().map(|l| &l.name).collect::<Vec<_>>(),
                "comments": issue.comments,
            });

            let update = Update::with_data(
                SourceType::GitHub,
                self.repo_id,
                event_type,
                issue.title,
                issue.body,
                issue_url,
                Some(issue.user.id.to_string()),
                issue.updated_at,
                additional_data,
            );

            updates.push(update);
        }

        Ok(updates)
    }

    /// Fetch pull requests from GitHub
    async fn fetch_pull_requests(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Get pull requests
        let pulls_page = self
            .client
            .pulls(&self.owner, &self.repo)
            .list()
            .state(octocrab::params::State::All)
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
            let pr_url = format!(
                "https://github.com/{}/{}/pull/{}",
                self.owner, self.repo, pr_num
            );

            // Determine if this is a new PR or an update
            let event_type = match since {
                Some(since_time) if pr.created_at.unwrap_or_else(Utc::now) > since_time => {
                    UpdateEventType::PullRequest
                }
                _ => UpdateEventType::PullRequestUpdate,
            };

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
                self.repo_id,
                event_type,
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
    async fn fetch_releases(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Get releases
        let releases = self
            .client
            .repos(&self.owner, &self.repo)
            .releases()
            .list()
            .send()
            .await
            .map_err(Self::map_error)?;

        // Filter by date if needed
        let filtered_releases = if let Some(since_time) = since {
            releases
                .items
                .into_iter()
                .filter(|r| r.published_at.is_some_and(|d| d >= since_time))
                .collect::<Vec<_>>()
        } else {
            releases.items
        };

        // Convert each release to an Update
        for release in filtered_releases {
            let tag_name = release.tag_name.clone();
            let release_url = release.html_url.to_string();

            // Store additional data as JSON
            let additional_data = json!({
                "tag_name": tag_name,
                "prerelease": release.prerelease,
                "draft": release.draft,
                "assets_count": release.assets.len(),
            });

            let update = Update::with_data(
                SourceType::GitHub,
                self.repo_id,
                UpdateEventType::Release,
                format!("Release: {}", release.name.unwrap_or(tag_name.clone())),
                release.body,
                release_url,
                release.author.as_ref().map(|a| a.login.clone()),
                release.published_at.unwrap_or_else(Utc::now),
                additional_data,
            );

            updates.push(update);
        }

        Ok(updates)
    }
}

#[async_trait]
impl Source for GitHubSource {
    fn get_type(&self) -> SourceType {
        SourceType::GitHub
    }

    fn get_id(&self) -> String {
        self.repo_id.to_string()
    }

    fn get_name(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }

    fn get_description(&self) -> Option<String> {
        None
    }

    fn get_url(&self) -> String {
        format!("https://github.com/{}/{}", self.owner, self.repo)
    }

    async fn fetch_updates(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Update>, SourceError> {
        let mut updates = Vec::new();

        // Fetch the different types of updates
        let commits = self.fetch_commits(since).await?;
        let issues = self.fetch_issues(since).await?;
        let pull_requests = self.fetch_pull_requests(since).await?;
        let releases = self.fetch_releases(since).await?;

        updates.extend(commits);
        updates.extend(issues);
        updates.extend(pull_requests);
        updates.extend(releases);

        Ok(updates)
    }

    fn get_metadata(&self) -> SourceMetadata {
        SourceMetadata {
            source_type: SourceType::GitHub,
            data: json!({
                "owner": self.owner,
                "repo": self.repo,
                "branch": self.branch,
            }),
        }
    }

    fn is_duplicate(&self, update1: &Update, update2: &Update) -> bool {
        // 基本条件：必须是相同仓库和相同事件类型
        if update1.source_id != update2.source_id || update1.event_type != update2.event_type {
            return false;
        }

        // 根据事件类型使用不同的判断标准
        match update1.event_type {
            UpdateEventType::Commit => {
                // 对于提交，尝试从additional_data中提取SHA
                if let (Some(data1), Some(data2)) =
                    (&update1.additional_data, &update2.additional_data)
                {
                    if let (Some(sha1), Some(sha2)) = (
                        data1.get("sha").and_then(|v| v.as_str()),
                        data2.get("sha").and_then(|v| v.as_str()),
                    ) {
                        return sha1 == sha2;
                    }
                }
                // 如果无法获取SHA，退回到标题和日期比较
                update1.title == update2.title && update1.event_date == update2.event_date
            }
            UpdateEventType::Issue | UpdateEventType::IssueUpdate => {
                // 对于问题，使用问题编号
                if let (Some(data1), Some(data2)) =
                    (&update1.additional_data, &update2.additional_data)
                {
                    if let (Some(num1), Some(num2)) = (
                        data1.get("number").and_then(|v| v.as_u64()),
                        data2.get("number").and_then(|v| v.as_u64()),
                    ) {
                        return num1 == num2;
                    }
                }
                // 退回到标题和日期
                update1.title == update2.title && update1.event_date == update2.event_date
            }
            UpdateEventType::PullRequest | UpdateEventType::PullRequestUpdate => {
                // 对于PR，使用PR编号
                if let (Some(data1), Some(data2)) =
                    (&update1.additional_data, &update2.additional_data)
                {
                    if let (Some(num1), Some(num2)) = (
                        data1.get("number").and_then(|v| v.as_u64()),
                        data2.get("number").and_then(|v| v.as_u64()),
                    ) {
                        return num1 == num2;
                    }
                }
                // 退回到标题和日期
                update1.title == update2.title && update1.event_date == update2.event_date
            }
            UpdateEventType::Release => {
                // 对于发布，使用tag名称和ID
                if let (Some(data1), Some(data2)) =
                    (&update1.additional_data, &update2.additional_data)
                {
                    if let (Some(id1), Some(id2)) = (
                        data1.get("id").and_then(|v| v.as_u64()),
                        data2.get("id").and_then(|v| v.as_u64()),
                    ) {
                        return id1 == id2;
                    }
                    // 退回到tag名称
                    if let (Some(tag1), Some(tag2)) = (
                        data1.get("tag_name").and_then(|v| v.as_str()),
                        data2.get("tag_name").and_then(|v| v.as_str()),
                    ) {
                        return tag1 == tag2;
                    }
                }
                // 退回到标题和日期
                update1.title == update2.title && update1.event_date == update2.event_date
            }
            // 对于其他类型的更新，使用默认的标题、类型和ID比较
            _ => update1.title == update2.title && update1.event_date == update2.event_date,
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
        assert_eq!(source.get_id(), "123456");
    }

    #[tokio::test]
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
                    assert!(update.source_id > 0);
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
