use std::{path::PathBuf, sync::Arc};

use chrono::{DateTime, Local, Utc};
use tokio::{
    fs::{File, create_dir_all},
    io::AsyncWriteExt,
};
use tracing::info;

use crate::{
    error::AppError,
    llm::{LLMClient, PromptBuilder},
    models::{Fetcher, Source, SourceType},
    notification::{NotificationManager, NotificationMessage},
    storage::{Storage, StorageError},
};

const DEFAULT_REPORT_DIR: &str = "docs/reports";

struct ReportResult {
    report_paths: String,
    report_data: String,
    ai_report_path: Option<String>,
    ai_report_data: Option<String>,
}

#[derive(Clone)]
pub struct Reporter<S: Storage> {
    storage: Arc<S>,
    report_dir: PathBuf,
    llm_client: Vec<Arc<dyn LLMClient>>,
    notification_manager: Option<Arc<NotificationManager>>,
}

impl<S: Storage> Reporter<S> {
    pub fn new(storage: Arc<S>, report_dir: Option<PathBuf>) -> Self {
        let report_dir = report_dir.unwrap_or_else(|| PathBuf::from(DEFAULT_REPORT_DIR));
        Self {
            storage,
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

        let source = self.build_source(source_type, source_id).await?;
        let source_name = source.get_name();
        let report_result = self.fetch_and_summary(&source, &model).await?;
        info!(
            "Generated reports for {}: {}",
            source_name, report_result.report_paths
        );

        // if there is ai report, send notification
        if let Some(ai_report_path) = &report_result.ai_report_path {
            self.send_notification(&source_name, ai_report_path, to_emails.clone())
                .await?;
        }
        Ok((
            report_result.report_data,
            report_result.ai_report_data.unwrap_or_default(),
        ))
    }

    async fn build_source(
        &self,
        source_type: SourceType,
        source_id: i32,
    ) -> Result<Source, AppError> {
        match source_type {
            SourceType::GitHub => {
                if let Some(repo) = self.storage.get_repository(source_id).await? {
                    Ok(Source::GitHub(repo))
                } else {
                    Err(StorageError::NotFound(source_type, source_id).into())
                }
            }
        }
    }

    async fn send_notification(
        &self,
        source_name: &str,
        report_path: &str,
        to_emails: Vec<String>,
    ) -> Result<(), AppError> {
        if let Some(notification_manager) = &self.notification_manager {
            let report_content = match tokio::fs::read_to_string(report_path).await {
                Ok(content) => content,
                Err(e) => return Err(AppError::AnyError(anyhow::anyhow!(e))),
            };

            let subject = format!("XiaoTian Update Report: {}", source_name);
            let message = NotificationMessage::new(subject, report_content);

            notification_manager.send(&message, to_emails).await?;
            Ok(())
        } else {
            Err(AppError::AnyError(anyhow::anyhow!(
                "No notification manager found"
            )))
        }
    }

    async fn fetch_and_summary(
        &self,
        source: &Source,
        model: &str,
    ) -> Result<ReportResult, AppError> {
        // fetch updates from 7 days ago to now
        let since = Local::now() - chrono::Duration::days(7);
        let updates = source.fetch_updates(since).await?;
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

        // create report directory if not exists
        let source_type = source.get_type();
        let source_type_dir = self.report_dir.join(source_type.to_string().to_lowercase());
        create_dir_all(&source_type_dir)
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;

        // save updates and generate summary
        let report_result = self
            .save_and_summary(source, &updates, since, model)
            .await?;

        Ok(report_result)
    }

    /// save updates and generate summary
    async fn save_and_summary(
        &self,
        source: &Source,
        report: &str,
        since: DateTime<Local>,
        model: &str,
    ) -> Result<ReportResult, AppError> {
        let source_name = source.get_name();
        let source_type = source.get_type();
        let source_type_str = source_type.to_string().to_lowercase();
        let since_str = since.format("%Y%m%d").to_string();
        let until_str = Local::now().format("%Y%m%d").to_string();
        let safe_name = source_name.replace('/', "_");

        // 1. save original report
        let original_filename = format!(
            "{}_{}_{}_{}.md",
            source_type_str, safe_name, since_str, until_str
        );
        let source_type_dir = self.report_dir.join(&source_type_str);
        let original_report_path = source_type_dir.join(&original_filename);

        // write original report
        self.save_report(&original_report_path, report)
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;
        info!(
            "Saved original report to {}",
            original_report_path.display()
        );

        // 2. if there is llm client, generate ai summary
        let mut result = ReportResult {
            report_paths: original_report_path.to_string_lossy().to_string(),
            report_data: report.to_string(),
            ai_report_path: None,
            ai_report_data: None,
        };

        if let Some(client) = self.get_llm_client(model) {
            let (ai_report_path, ai_report_data) = self
                .summary_by_llm(source, report, &original_filename, since, client)
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

    /// call llm to generate summary and save
    async fn summary_by_llm(
        &self,
        source: &Source,
        report: &str,
        original_filename: &str,
        since: DateTime<Local>,
        llm_client: &Arc<dyn LLMClient>,
    ) -> Result<(String, String), AppError> {
        // 1. format updates content to llm input
        let updates_content = report;

        // 2. build prompt and call llm
        let prompt = PromptBuilder::build_report_summary(updates_content);
        let summary = llm_client.generate(&prompt, false).await?;

        info!("Generated AI summary for {}", source.get_name());

        // 3. generate ai summary report
        let source_type = source.get_type().to_string().to_lowercase();
        let source_name = source.get_name().replace('/', "_");
        let since_str = since.format("%Y%m%d").to_string();
        let until_str = Local::now().format("%Y%m%d").to_string();

        let ai_report_filename = format!(
            "{}_{}_{}_{}_report.md",
            source_type, source_name, since_str, until_str
        );

        // 4. generate ai summary report content
        let ai_report = self.generate_ai_report(
            source,
            &summary,
            original_filename,
            since,
            Some(llm_client.get_name()),
        );

        // 5. save ai summary report
        let source_type_dir = self.report_dir.join(&source_type);
        let ai_report_path = source_type_dir.join(&ai_report_filename);

        self.save_report(&ai_report_path, &ai_report).await?;

        Ok((ai_report_path.to_string_lossy().to_string(), ai_report))
    }

    /// save report content to file
    async fn save_report(&self, path: &PathBuf, content: &str) -> Result<(), AppError> {
        let mut file = File::create(path)
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;

        file.write_all(content.as_bytes())
            .await
            .map_err(|e| AppError::AnyError(anyhow::anyhow!(e)))?;

        Ok(())
    }

    /// generate concise ai summary report
    fn generate_ai_report(
        &self,
        source: &Source,
        ai_summary: &str,
        original_report_filename: &str,
        since: DateTime<Local>,
        model: Option<&str>,
    ) -> String {
        let mut report = String::new();

        // title
        report.push_str(&format!("# AI Summary: {}\n\n", source.get_name()));
        report.push_str(&format!(
            "## Period: {} to {}\n\n",
            since.format("%Y-%m-%d"),
            Local::now().format("%Y-%m-%d")
        ));

        // source information
        report.push_str("## Source Information\n\n");
        report.push_str(&format!("- **Type**: {}\n", source.get_type()));
        report.push_str(&format!("- **Name**: {}\n", source.get_name()));
        report.push('\n');

        // ai summary part
        report.push_str("## AI Generated Summary\n\n");
        report.push_str(ai_summary);
        report.push_str("\n\n");

        // metadata information
        report.push_str("## Report Metadata\n\n");
        report.push_str(&format!(
            "- **Generated At**: {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if let Some(model) = model {
            report.push_str(&format!("- **LLM Provider**: {}\n", model));
        }

        // reference to original report
        report.push_str(&format!(
            "- **Detailed Report**: [{}]({})\n",
            original_report_filename, original_report_filename
        ));

        report.push('\n');

        // add footer
        report.push_str(&format!(
            "---\nGenerated by XiaoTian with AI assistance at {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));

        report
    }
}
