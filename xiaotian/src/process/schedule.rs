//! Cron scheduler handler

use std::{path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use tokio::{
    fs::{File, create_dir_all},
    io::AsyncWriteExt,
};
use tracing::{error, info};

use crate::{
    error::AppError,
    llm::{LLMClient, PromptBuilder},
    models::{Source, SourceConfig, SourceFactory, SourceType, Update, UpdateEventType},
    notification::{NotificationManager, NotificationMessage},
    storage::{RepositoryStorage, StorageError},
};

const DEFAULT_REPORT_DIR: &str = "docs/reports";

/// 报告结果
struct ReportResult {
    report_paths: String,
    report_data: String,
    ai_report_path: Option<String>,
    ai_report_data: Option<String>,
}

#[derive(Clone)]
pub struct ScheduleHandler<S: RepositoryStorage> {
    storage: Arc<S>,
    source_factory: Arc<dyn SourceFactory>,
    report_dir: PathBuf,
    llm_client: Vec<Arc<dyn LLMClient>>,
    notification_manager: Option<Arc<NotificationManager>>,
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
            llm_client: Vec::new(),
            notification_manager: None,
        }
    }

    pub fn with_llm_client(mut self, llm_client: Arc<dyn LLMClient>) -> Self {
        self.llm_client.push(llm_client);
        self
    }

    pub fn get_llm_client(&self, model: &str) -> Option<&Arc<dyn LLMClient>> {
        self.llm_client
            .iter()
            .find(|client| client.get_name() == model)
    }

    pub fn available_models(&self) -> Vec<String> {
        self.llm_client
            .iter()
            .map(|client| client.get_name().to_string())
            .collect()
    }

    pub fn with_notification_manager(
        mut self,
        notification_manager: Arc<NotificationManager>,
    ) -> Self {
        self.notification_manager = Some(notification_manager);
        self
    }

    pub async fn run_single(
        &self,
        source_type: SourceType,
        source_id: i32,
        model: String,
        to_emails: Vec<String>,
    ) -> Result<(String, String), AppError> {
        info!(
            "Running scheduled update for source_type: {}, source_id: {}, using model: {}, to_emails: {}",
            source_type,
            source_id,
            model,
            to_emails.join(", ")
        );

        let repo = self.storage.get_repository(source_id).await?;
        if repo.is_none() {
            return Err(
                StorageError::NotFound(source_type.to_string(), source_id.to_string()).into(),
            );
        }
        let repo = repo.unwrap();

        let config = SourceConfig {
            source_type: SourceType::GitHub,
            config: serde_json::json!({
                "owner": repo.owner,
                "repo": repo.name,
            }),
        };

        let source = self.source_factory.create_source(config, repo.id).await?;
        let report_result = self.fetch_and_summary(source.as_ref(), &model).await?;
        info!(
            "Generated reports for {}/{}: {}",
            repo.owner, repo.name, report_result.report_paths
        );

        // 如果有AI摘要报告，发送通知
        if let Some(ai_report_path) = &report_result.ai_report_path {
            self.send_notification(&repo.owner, &repo.name, ai_report_path, to_emails.clone())
                .await?;
        }
        Ok((
            report_result.report_data,
            report_result.ai_report_data.unwrap_or_default(),
        ))
    }

    pub async fn run(&self, model: &str, to_emails: Vec<String>) -> Result<(), AppError> {
        info!("Running scheduled update for all repositories...");
        self.update_all_repositories(model, to_emails).await?;
        Ok(())
    }

    async fn update_all_repositories(
        &self,
        model: &str,
        to_emails: Vec<String>,
    ) -> Result<usize, AppError> {
        let repositories = self.storage.get_all_repositories().await?;

        if repositories.is_empty() {
            info!("No repositories found to update");
            return Ok(0);
        }

        info!("Found {} repositories to update", repositories.len());
        let mut success_count = 0;

        for repo in repositories {
            info!(
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
                Ok(source) => match self.fetch_and_summary(source.as_ref(), model).await {
                    Ok(report_result) => {
                        info!(
                            "Generated reports for {}/{}: {}",
                            repo.owner, repo.name, report_result.report_paths
                        );

                        // 如果有AI摘要报告，发送通知
                        if let Some(ai_report_path) = &report_result.ai_report_path {
                            self.send_notification(
                                &repo.owner,
                                &repo.name,
                                ai_report_path,
                                to_emails.clone(),
                            )
                            .await?;
                        }

                        success_count += 1;
                    }
                    Err(e) => error!(
                        "Failed to generate reports for {}/{}: {}",
                        repo.owner, repo.name, e
                    ),
                },
                Err(e) => error!(
                    "Failed to create source for {}/{}: {}",
                    repo.owner, repo.name, e
                ),
            }
        }

        Ok(success_count)
    }

    /// 发送通知
    async fn send_notification(
        &self,
        owner: &str,
        repo: &str,
        report_path: &str,
        to_emails: Vec<String>,
    ) -> Result<(), AppError> {
        if let Some(notification_manager) = &self.notification_manager {
            // 读取报告内容
            let report_content = match tokio::fs::read_to_string(report_path).await {
                Ok(content) => content,
                Err(e) => return Err(AppError::AnyError(anyhow::anyhow!(e))),
            };

            // 创建通知消息
            let subject = format!("XiaoTian Update Report: {}/{}", owner, repo);
            let message = NotificationMessage::new(subject, report_content);

            // 发送通知
            notification_manager.send(&message, to_emails).await?;
            Ok(())
        } else {
            Err(AppError::AnyError(anyhow::anyhow!(
                "No notification manager found"
            )))
        }
    }

    /// 获取更新并生成摘要
    async fn fetch_and_summary(
        &self,
        source: &dyn Source,
        model: &str,
    ) -> Result<ReportResult, AppError> {
        // 获取7天内的更新
        let since = Utc::now() - chrono::Duration::days(7);
        let updates = source.fetch_updates(Some(since)).await?;

        if updates.is_empty() {
            info!("No updates found for source {}", source.get_name());
            return Ok(ReportResult {
                report_paths: "No updates found".to_string(),
                report_data: "No updates found".to_string(),
                ai_report_path: None,
                ai_report_data: None,
            });
        }

        info!("Found {} updates for {}", updates.len(), source.get_name());

        // 确保报告目录存在
        let source_type = source.get_type();
        let source_type_dir = self.report_dir.join(source_type.to_string().to_lowercase());
        create_dir_all(&source_type_dir)
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;

        // 为所有更新生成报告
        let report_result = self
            .save_and_summary(source, &updates, since, Utc::now(), model)
            .await?;

        Ok(report_result)
    }

    /// 保存原始信息并生成摘要
    async fn save_and_summary(
        &self,
        source: &dyn Source,
        updates: &[Update],
        since: DateTime<Utc>,
        until: DateTime<Utc>,
        model: &str,
    ) -> Result<ReportResult, AppError> {
        let source_name = source.get_name();
        let source_type = source.get_type();
        let source_type_str = source_type.to_string().to_lowercase();
        let since_str = since.format("%Y%m%d").to_string();
        let until_str = until.format("%Y%m%d").to_string();
        let safe_name = source_name.replace('/', "_");

        // 1. 保存原始报告
        let original_filename = format!(
            "{}_{}_{}_{}.md",
            source_type_str, safe_name, since_str, until_str
        );

        let source_type_dir = self.report_dir.join(&source_type_str);
        let original_report_path = source_type_dir.join(&original_filename);
        let original_report = self.generate_original_report(source, updates, since, until)?;

        // 写入原始报告
        self.save_report(&original_report_path, &original_report)
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;
        info!(
            "Saved original report to {}",
            original_report_path.display()
        );

        // 2. 如果有LLM客户端，生成AI摘要
        let mut result = ReportResult {
            report_paths: original_report_path.to_string_lossy().to_string(),
            report_data: original_report,
            ai_report_path: None,
            ai_report_data: None,
        };

        if let Some(client) = self.get_llm_client(model) {
            let (ai_report_path, ai_report_data) = self
                .summary_by_llm(source, updates, &original_filename, since, until, client)
                .await?;
            result.report_paths = format!("{} and {}", result.report_paths, ai_report_path);
            result.ai_report_path = Some(ai_report_path);
            result.ai_report_data = Some(ai_report_data);
        } else {
            return Err(AppError::AnyError(anyhow::anyhow!(
                "LLM client for model {} not found",
                model
            )));
        }

        Ok(result)
    }

    /// 调用LLM生成摘要并保存
    async fn summary_by_llm(
        &self,
        source: &dyn Source,
        updates: &[Update],
        original_filename: &str,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
        llm_client: &Arc<dyn LLMClient>,
    ) -> Result<(String, String), AppError> {
        // 1. 格式化更新内容为LLM输入
        let updates_content = self.format_updates_for_llm(updates);

        // 2. 构建提示词并调用LLM
        let prompt = PromptBuilder::build_report_summary(&updates_content);
        let summary = llm_client.generate(&prompt, false).await?;

        info!("Generated AI summary for {}", source.get_name());

        // 3. 生成AI摘要报告
        let source_type = source.get_type().to_string().to_lowercase();
        let source_name = source.get_name().replace('/', "_");
        let since_str = since.format("%Y%m%d").to_string();
        let until_str = until.format("%Y%m%d").to_string();

        let ai_report_filename = format!(
            "{}_{}_{}_{}_report.md",
            source_type, source_name, since_str, until_str
        );

        // 4. 生成AI摘要报告内容
        let ai_report = self.generate_ai_report(
            source,
            &summary,
            original_filename,
            since,
            until,
            Some(llm_client.get_name()),
        );

        // 5. 保存AI摘要报告
        let source_type_dir = self.report_dir.join(&source_type);
        let ai_report_path = source_type_dir.join(&ai_report_filename);

        self.save_report(&ai_report_path, &ai_report).await?;

        Ok((ai_report_path.to_string_lossy().to_string(), ai_report))
    }

    /// 格式化更新内容给LLM使用
    fn format_updates_for_llm(&self, updates: &[Update]) -> String {
        let mut updates_content = String::new();

        for update in updates {
            match update.event_type {
                UpdateEventType::PullRequest => {
                    updates_content.push_str(&format!("## PR: {}\n", update.title));
                }
                UpdateEventType::Issue => {
                    updates_content.push_str(&format!("## Issue: {}\n", update.title));
                }
                _ => continue,
            }

            if let Some(desc) = &update.description {
                updates_content.push_str(&format!("{}\n\n", desc));
            }

            if let Some(author) = &update.author {
                updates_content.push_str(&format!("作者: {}\n", author));
            }

            updates_content.push_str(&format!("链接: {}\n\n", update.url));
        }

        updates_content
    }

    /// 保存报告内容到文件
    async fn save_report(&self, path: &PathBuf, content: &str) -> Result<(), AppError> {
        let mut file = File::create(path)
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;

        file.write_all(content.as_bytes())
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;

        Ok(())
    }

    /// 生成原始报告内容
    fn generate_original_report(
        &self,
        source: &dyn Source,
        updates: &[Update],
        since: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<String, AppError> {
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

        for update in updates {
            match update.event_type {
                UpdateEventType::PullRequest => prs.push(update),
                UpdateEventType::Issue => issues.push(update),
                _ => continue,
            }
        }

        report.push_str(&format!("- **Total Updates**: {}\n", updates.len()));
        report.push_str(&format!("- **Pull Requests**: {}\n", prs.len()));
        report.push_str(&format!("- **Issues**: {}\n", issues.len()));
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

        // footer
        report.push_str(&format!(
            "---\nGenerated by XiaoTian at {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));

        Ok(report)
    }

    /// 生成简洁的AI摘要报告
    fn generate_ai_report(
        &self,
        source: &dyn Source,
        ai_summary: &str,
        original_report_filename: &str,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
        model: Option<&str>,
    ) -> String {
        let mut report = String::new();

        // 标题
        report.push_str(&format!("# AI Summary: {}\n\n", source.get_name()));
        report.push_str(&format!(
            "## Period: {} to {}\n\n",
            since.format("%Y-%m-%d"),
            until.format("%Y-%m-%d")
        ));

        // 源信息
        report.push_str("## Source Information\n\n");
        report.push_str(&format!("- **Type**: {}\n", source.get_type()));
        report.push_str(&format!("- **Name**: {}\n", source.get_name()));
        if let Some(desc) = source.get_description() {
            report.push_str(&format!("- **Description**: {}\n", desc));
        }
        report.push_str(&format!("- **URL**: {}\n", source.get_url()));
        report.push('\n');

        // AI 摘要部分
        report.push_str("## AI Generated Summary\n\n");
        report.push_str(ai_summary);
        report.push_str("\n\n");

        // 元数据信息
        report.push_str("## Report Metadata\n\n");
        report.push_str(&format!(
            "- **Generated At**: {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if let Some(model) = model {
            report.push_str(&format!("- **LLM Provider**: {}\n", model));
        }

        // 添加原始报告的引用
        report.push_str(&format!(
            "- **Detailed Report**: [{}]({})\n",
            original_report_filename, original_report_filename
        ));

        report.push('\n');

        // 添加页脚
        report.push_str(&format!(
            "---\nGenerated by XiaoTian with AI assistance at {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));

        report
    }
}
