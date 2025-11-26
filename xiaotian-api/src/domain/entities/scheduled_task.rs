use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::config::{SummaryLength, EmailConfig, EmailTemplate};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "scheduled_tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: String, // String ID like "task-daily-tech"

    #[sea_orm(column_type = "String(Some(200))", not_null)]
    pub name: String,

    #[sea_orm(column_type = "Boolean", not_null, default_value = "false")]
    pub enabled: bool,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub cron_expression: String,

    #[sea_orm(column_type = "String(Some(200))", nullable)]
    pub cron_description: Option<String>,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub next_run: Option<DateTime<Utc>>,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub last_run: Option<DateTime<Utc>>,

    #[sea_orm(column_type = "String(Some(20))", nullable)]
    pub last_run_status: Option<String>, // "success", "failed"

    #[sea_orm(column_type = "Integer", nullable)]
    pub last_run_duration: Option<i32>, // in seconds

    #[sea_orm(column_type = "JsonBinary", not_null)]
    pub email_config: serde_json::Value,

    #[sea_orm(column_type = "JsonBinary", not_null)]
    pub selected_feeds: serde_json::Value, // Array of feed IDs

    #[sea_orm(column_type = "Boolean", not_null, default_value = "true")]
    pub ai_summary_enabled: bool,

    #[sea_orm(column_type = "String(Some(20))", not_null, default_value = "\"medium\"")]
    pub summary_length: String, // "short", "medium", "long"

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTime<Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::scheduled_task_feed::Entity")]
    TaskFeeds,

    #[sea_orm(has_many = "super::task_execution::Entity")]
    Executions,
}

impl Related<super::scheduled_task_feed::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskFeeds.def()
    }
}

impl Related<super::task_execution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Executions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let task_id = format!("task-{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        Self {
            id: Set(task_id),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            email_config: Set(serde_json::to_value(EmailConfig::default()).unwrap()),
            selected_feeds: Set(serde_json::json!([])),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Utc::now());
        Ok(self)
    }
}

impl Model {
    pub fn email_config(&self) -> EmailConfig {
        serde_json::from_value(self.email_config.clone()).unwrap_or_default()
    }

    pub fn set_email_config(&mut self, config: EmailConfig) {
        self.email_config = serde_json::to_value(config).unwrap_or(serde_json::Value::Null);
        self.updated_at = Utc::now();
    }

    pub fn selected_feeds(&self) -> Vec<i32> {
        serde_json::from_value(self.selected_feeds.clone()).unwrap_or_default()
    }

    pub fn set_selected_feeds(&mut self, feeds: Vec<i32>) {
        self.selected_feeds = serde_json::to_value(feeds).unwrap_or(serde_json::Value::Null);
        self.updated_at = Utc::now();
    }

    pub fn summary_length(&self) -> SummaryLength {
        match self.summary_length.as_str() {
            "short" => SummaryLength::Short,
            "long" => SummaryLength::Long,
            _ => SummaryLength::Medium,
        }
    }

    pub fn set_summary_length(&mut self, length: SummaryLength) {
        self.summary_length = match length {
            SummaryLength::Short => "short".to_string(),
            SummaryLength::Long => "long".to_string(),
            SummaryLength::Medium => "medium".to_string(),
        };
        self.updated_at = Utc::now();
    }

    pub fn last_run_status(&self) -> Option<TaskExecutionStatus> {
        self.last_run_status.as_ref().map(|status| match status.as_str() {
            "success" => TaskExecutionStatus::Success,
            "failed" => TaskExecutionStatus::Failed,
            _ => TaskExecutionStatus::Failed,
        })
    }

    pub fn set_last_run_status(&mut self, status: TaskExecutionStatus, duration: i32) {
        self.last_run_status = Some(match status {
            TaskExecutionStatus::Success => "success".to_string(),
            TaskExecutionStatus::Failed => "failed".to_string(),
        });
        self.last_run_duration = Some(duration);
        self.last_run = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.updated_at = Utc::now();
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = Utc::now();
    }

    pub fn has_feed(&self, feed_id: i32) -> bool {
        self.selected_feeds().contains(&feed_id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskExecutionStatus {
    Success,
    Failed,
}

impl ToString for TaskExecutionStatus {
    fn to_string(&self) -> String {
        match self {
            TaskExecutionStatus::Success => "success".to_string(),
            TaskExecutionStatus::Failed => "failed".to_string(),
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateScheduledTaskRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,

    #[validate(custom = "validate_cron_expression")]
    pub cron_expression: String,

    #[validate(custom = "validate_email_config")]
    pub email_config: EmailConfig,

    #[validate(length(min = 1, message = "At least one feed must be selected"))]
    pub selected_feeds: Vec<i32>,

    pub ai_summary_enabled: bool,

    #[validate(custom = "validate_summary_length")]
    pub summary_length: String,

    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct UpdateScheduledTaskRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,

    pub enabled: Option<bool>,

    #[validate(custom = "validate_cron_expression")]
    pub cron_expression: Option<String>,

    #[validate(custom = "validate_email_config")]
    pub email_config: Option<EmailConfig>,

    pub selected_feeds: Option<Vec<i32>>,

    pub ai_summary_enabled: Option<bool>,

    #[validate(custom = "validate_summary_length")]
    pub summary_length: Option<String>,
}

fn validate_cron_expression(expression: &str) -> Result<(), validator::ValidationError> {
    crate::shared::validate_cron_expression(expression)
        .map_err(|_| validator::ValidationError::new("invalid_cron_expression"))
}

fn validate_email_config(config: &EmailConfig) -> Result<(), validator::ValidationError> {
    crate::shared::validate_email_addresses(&config.recipient_emails)
        .map_err(|_| validator::ValidationError::new("invalid_email_config"))
}

fn validate_summary_length(length: &str) -> Result<(), validator::ValidationError> {
    if !matches!(length, "short" | "medium" | "long") {
        return Err(validator::ValidationError::new("invalid_summary_length"));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteTaskResponse {
    pub task_id: String,
    pub execution_id: String,
    pub start_time: DateTime<Utc>,
    pub status: String, // "started"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecutionParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EmailConfig;

    #[test]
    fn test_scheduled_task_properties() {
        let mut task = Model {
            id: "test-task".to_string(),
            name: "Test Task".to_string(),
            enabled: false,
            cron_expression: "0 9 * * *".to_string(),
            cron_description: None,
            next_run: None,
            last_run: None,
            last_run_status: None,
            last_run_duration: None,
            email_config: serde_json::to_value(EmailConfig::default()).unwrap(),
            selected_feeds: serde_json::json!([1, 2, 3]),
            ai_summary_enabled: true,
            summary_length: "medium".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(!task.is_enabled());
        task.enable();
        assert!(task.is_enabled());

        let feeds = task.selected_feeds();
        assert_eq!(feeds.len(), 3);
        assert!(feeds.contains(&1));
        assert!(task.has_feed(2));

        assert_eq!(task.summary_length(), SummaryLength::Medium);
        task.set_summary_length(SummaryLength::Short);
        assert_eq!(task.summary_length(), SummaryLength::Short);

        assert!(task.last_run_status().is_none());
        task.set_last_run_status(TaskExecutionStatus::Success, 120);
        assert!(task.last_run_status().is_some());
        assert_eq!(task.last_run_duration, Some(120));
    }

    #[test]
    fn test_create_scheduled_task_request_validation() {
        let valid_request = CreateScheduledTaskRequest {
            name: "Daily Tech Summary".to_string(),
            cron_expression: "0 9 * * *".to_string(),
            email_config: EmailConfig::default(),
            selected_feeds: vec![1, 2],
            ai_summary_enabled: true,
            summary_length: "medium".to_string(),
            enabled: Some(true),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateScheduledTaskRequest {
            name: "".to_string(), // Invalid: empty name
            cron_expression: "invalid".to_string(), // Invalid: invalid cron
            email_config: EmailConfig {
                recipient_emails: vec!["invalid-email".to_string()],
                ..Default::default()
            },
            selected_feeds: vec![], // Invalid: no feeds
            ai_summary_enabled: true,
            summary_length: "invalid".to_string(), // Invalid: invalid length
            enabled: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_task_execution_status() {
        let status = TaskExecutionStatus::Success;
        assert_eq!(status.to_string(), "success");

        let failed_status = TaskExecutionStatus::Failed;
        assert_eq!(failed_status.to_string(), "failed");
    }

    #[test]
    fn test_cron_expression_validation() {
        assert!(validate_cron_expression("0 9 * * *").is_ok());
        assert!(validate_cron_expression("*/5 * * * *").is_ok());
        assert!(validate_cron_expression("0 9 * * 1-5").is_ok());

        assert!(validate_cron_expression("").is_err());
        assert!(validate_cron_expression("invalid").is_err());
        assert!(validate_cron_expression("0 25 * * *").is_err()); // Invalid hour
    }

    #[test]
    fn test_summary_length_validation() {
        assert!(validate_summary_length("short").is_ok());
        assert!(validate_summary_length("medium").is_ok());
        assert!(validate_summary_length("long").is_ok());

        assert!(validate_summary_length("invalid").is_err());
        assert!(validate_summary_length("").is_err());
        assert!(validate_summary_length("very_long").is_err());
    }
}