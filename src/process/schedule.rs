//! Schedule command handler

use std::{path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use tokio::{
    fs::{File, create_dir_all},
    io::AsyncWriteExt,
};

use crate::{
    models::{Source, SourceConfig, SourceFactory, SourceType, Update, UpdateEventType},
    storage::RepositoryStorage,
};

const DEFAULT_REPORT_DIR: &str = "docs/reports";

/// 调度处理器
#[derive(Clone)]
pub struct ScheduleHandler<S: RepositoryStorage> {
    storage: Arc<S>,
    source_factory: Arc<dyn SourceFactory>,
    report_dir: PathBuf,
}

impl<S: RepositoryStorage> ScheduleHandler<S> {
    pub fn new(
        storage: Arc<S>,
        source_factory: Arc<dyn SourceFactory>,
        report_dir: Option<PathBuf>,
    ) -> Self {
        let report_dir = report_dir.unwrap_or_else(|| PathBuf::from(DEFAULT_REPORT_DIR));
        Self {
            storage,
            source_factory,
            report_dir,
        }
    }

    pub async fn run(&self) {
        println!("Running scheduled update for all repositories...");
        match self.update_all_repositories().await {
            Ok(count) => println!("Successfully updated {} repositories", count),
            Err(e) => println!("Error updating repositories: {}", e),
        }
    }

    async fn update_all_repositories(&self) -> Result<usize, String> {
        let repositories = self
            .storage
            .get_all_repositories()
            .await
            .map_err(|e| format!("Failed to get repositories: {}", e))?;

        if repositories.is_empty() {
            println!("No repositories found to update");
            return Ok(0);
        }

        println!("Found {} repositories to update", repositories.len());
        let mut success_count = 0;

        for repo in repositories {
            println!(
                "Processing repository: {}/{} (ID: {})",
                repo.owner, repo.name, repo.id
            );

            let config = SourceConfig {
                source_type: SourceType::GitHub,
                config: serde_json::json!({
                    "owner": repo.owner,
                    "repo": repo.name,
                }),
            };

            match self.source_factory.create_source(config, repo.id).await {
                Ok(source) => match self.fetch_and_report(source.as_ref(), repo.id).await {
                    Ok(report_path) => {
                        println!(
                            "Generated report for {}/{}: {}",
                            repo.owner, repo.name, report_path
                        );
                        success_count += 1;
                    }
                    Err(e) => println!(
                        "Failed to generate report for {}/{}: {}",
                        repo.owner, repo.name, e
                    ),
                },
                Err(e) => println!(
                    "Failed to create source for {}/{}: {}",
                    repo.owner, repo.name, e
                ),
            }
        }

        Ok(success_count)
    }

    /// fetch updates and generate report
    async fn fetch_and_report(&self, source: &dyn Source, repo_id: i32) -> Result<String, String> {
        let source_type = source.get_type();
        let source_name = source.get_name();

        let since = Utc::now() - chrono::Duration::days(7);
        let updates = source
            .fetch_updates(Some(since))
            .await
            .map_err(|e| format!("Failed to fetch updates: {}", e))?;

        if updates.is_empty() {
            println!("No updates found for repository {}", repo_id);
            return Ok("No updates found".to_string());
        }

        // create report directory structure
        // docs/reports/{source_type}/{owner}_{repo}_{since}_{until}.md
        let source_type_dir = self.report_dir.join(source_type.to_string().to_lowercase());
        create_dir_all(&source_type_dir)
            .await
            .map_err(|e| format!("Failed to create source type directory: {}", e))?;

        let since_str = since.format("%Y%m%d").to_string();
        let until_str = Utc::now().format("%Y%m%d").to_string();
        let safe_name = source_name.replace('/', "_");
        let filename = format!(
            "{}_{}_{}_{}.md",
            source_type.to_string().to_lowercase(),
            safe_name,
            since_str,
            until_str
        );
        let report_path = source_type_dir.join(filename);

        // generate report
        let report_content = self
            .generate_markdown_report(source, &updates, since, Utc::now())
            .map_err(|e| format!("Failed to generate report: {}", e))?;

        // write report to file
        let mut file = File::create(&report_path)
            .await
            .map_err(|e| format!("Failed to create report file: {}", e))?;
        file.write_all(report_content.as_bytes())
            .await
            .map_err(|e| format!("Failed to write report file: {}", e))?;

        Ok(report_path.to_string_lossy().to_string())
    }

    /// generate markdown report
    fn generate_markdown_report(
        &self,
        source: &dyn Source,
        updates: &[Update],
        since: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<String, String> {
        let mut report = String::new();

        // title
        report.push_str(&format!("# Update Report: {}\n\n", source.get_name()));
        report.push_str(&format!(
            "## Period: {} to {}\n\n",
            since.format("%Y-%m-%d"),
            until.format("%Y-%m-%d")
        ));

        // source information
        report.push_str("## Source Information\n\n");
        report.push_str(&format!("- **Type**: {}\n", source.get_type()));
        report.push_str(&format!("- **Name**: {}\n", source.get_name()));
        if let Some(desc) = source.get_description() {
            report.push_str(&format!("- **Description**: {}\n", desc));
        }
        report.push_str(&format!("- **URL**: {}\n", source.get_url()));
        report.push('\n');

        // summary
        report.push_str("## Summary\n\n");
        let mut prs = Vec::new();
        let mut issues = Vec::new();
        let mut releases = Vec::new();

        for update in updates {
            match update.event_type {
                UpdateEventType::PullRequest => prs.push(update),
                UpdateEventType::Issue => issues.push(update),
                UpdateEventType::Release => releases.push(update),
                _ => (),
            }
        }

        report.push_str(&format!("- **Total Updates**: {}\n", updates.len()));
        report.push_str(&format!("- **Pull Requests**: {}\n", prs.len()));
        report.push_str(&format!("- **Issues**: {}\n", issues.len()));
        report.push_str(&format!("- **Releases**: {}\n", releases.len()));
        report.push('\n');

        // pull requests details
        if !prs.is_empty() {
            report.push_str("## Pull Requests\n\n");
            for pr in prs {
                report.push_str(&format!("### {}\n\n", pr.title));
                report.push_str(&format!(
                    "- **Date**: {}\n",
                    pr.event_date.format("%Y-%m-%d %H:%M:%S")
                ));
                if let Some(author) = &pr.author {
                    report.push_str(&format!("- **Author**: {}\n", author));
                }
                report.push_str(&format!("- **URL**: {}\n", pr.url));
                if let Some(desc) = &pr.description {
                    report.push_str(&format!("- **Description**:\n\n{}\n", desc));
                }
                report.push('\n');
            }
        }

        // issues details
        if !issues.is_empty() {
            report.push_str("## Issues\n\n");
            for issue in issues {
                report.push_str(&format!("### {}\n\n", issue.title));
                report.push_str(&format!(
                    "- **Date**: {}\n",
                    issue.event_date.format("%Y-%m-%d %H:%M:%S")
                ));
                if let Some(author) = &issue.author {
                    report.push_str(&format!("- **Author**: {}\n", author));
                }
                report.push_str(&format!("- **URL**: {}\n", issue.url));
                if let Some(desc) = &issue.description {
                    report.push_str(&format!("- **Description**:\n\n{}\n", desc));
                }
                report.push('\n');
            }
        }

        // releases details
        if !releases.is_empty() {
            report.push_str("## Releases\n\n");
            for release in releases {
                report.push_str(&format!("### {}\n\n", release.title));
                report.push_str(&format!(
                    "- **Date**: {}\n",
                    release.event_date.format("%Y-%m-%d %H:%M:%S")
                ));
                if let Some(author) = &release.author {
                    report.push_str(&format!("- **Author**: {}\n", author));
                }
                report.push_str(&format!("- **URL**: {}\n", release.url));
                if let Some(desc) = &release.description {
                    report.push_str(&format!("- **Description**:\n\n{}\n", desc));
                }
                report.push('\n');
            }
        }

        // footer
        report.push_str(&format!(
            "---\nGenerated by XiaoTian at {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));

        Ok(report)
    }
}
