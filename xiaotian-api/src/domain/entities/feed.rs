use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "feeds")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub name: String,

    #[sea_orm(column_type = "String(Some(20))", not_null)]
    pub feed_type: String, // "rss"

    #[sea_orm(column_type = "String(Some(500))", not_null)]
    pub feed_url: String,

    #[sea_orm(column_type = "String(Some(500))", nullable)]
    pub description: Option<String>,

    #[sea_orm(column_type = "String(Some(50))", nullable)]
    pub category: Option<String>,

    #[sea_orm(column_type = "String(Some(10))", not_null, default_value = "\"loading\"")]
    pub status: String, // "active", "loading", "error"

    #[sea_orm(column_type = "String(Some(10))", nullable)]
    pub icon: Option<String>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTime<Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub last_updated: Option<DateTime<Utc>>,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub summary_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::summary::Entity")]
    Summaries,

    #[sea_orm(has_many = "super::scheduled_task_feed::Entity")]
    ScheduledTaskFeeds,
}

impl Related<super::summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Summaries.def()
    }
}

impl Related<super::scheduled_task_feed::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduledTaskFeeds.def()
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateFeedRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,

    #[validate(custom = "validate_feed_type")]
    pub feed_type: String,

    #[validate(url(message = "Feed URL must be a valid URL"))]
    pub feed_url: String,

    #[validate(length(max = 500, message = "Description must be at most 500 characters"))]
    pub description: Option<String>,

    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: Option<String>,

    #[validate(length(max = 10, message = "Icon must be at most 10 characters"))]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct UpdateFeedRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,

    #[validate(custom = "validate_feed_type")]
    pub feed_type: Option<String>,

    #[validate(length(max = 500, message = "Description must be at most 500 characters"))]
    pub description: Option<String>,

    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: Option<String>,

    #[validate(length(max = 10, message = "Icon must be at most 10 characters"))]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedStats {
    pub total_summaries: i64,
    pub last_30_day_summaries: i64,
    pub avg_summaries_per_day: f64,
    pub last_sync_duration: Option<i64>, // in seconds
}

impl From<crate::shared::utils::EmailValidation> for validator::ValidationError {
    fn from(_validation: crate::shared::utils::EmailValidation) -> Self {
        validator::ValidationError::new("invalid_email")
    }
}

fn validate_feed_type(feed_type: &str) -> Result<(), validator::ValidationError> {
    if feed_type != "rss" {
        return Err(validator::ValidationError::new("invalid_feed_type"));
    }
    Ok(())
}

impl Model {
    pub fn status(&self) -> FeedStatus {
        match self.status.as_str() {
            "active" => FeedStatus::Active,
            "loading" => FeedStatus::Loading,
            "error" => FeedStatus::Error,
            _ => FeedStatus::Error, // Default to Error for unknown status
        }
    }

    pub fn set_status(&mut self, status: FeedStatus) {
        self.status = status.to_string();
        self.last_updated = Some(Utc::now());
    }

    pub fn is_active(&self) -> bool {
        self.status() == FeedStatus::Active
    }

    pub fn is_loading(&self) -> bool {
        self.status() == FeedStatus::Loading
    }

    pub fn has_error(&self) -> bool {
        self.status() == FeedStatus::Error
    }

    pub fn increment_summary_count(&mut self) {
        self.summary_count += 1;
        self.last_updated = Some(Utc::now());
    }

    pub fn decrement_summary_count(&mut self) {
        if self.summary_count > 0 {
            self.summary_count -= 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeedStatus {
    Active,
    Loading,
    Error,
}

impl ToString for FeedStatus {
    fn to_string(&self) -> String {
        match self {
            FeedStatus::Active => "active".to_string(),
            FeedStatus::Loading => "loading".to_string(),
            FeedStatus::Error => "error".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_feed_request_validation() {
        let valid_request = CreateFeedRequest {
            name: "Test Feed".to_string(),
            feed_type: "rss".to_string(),
            feed_url: "https://example.com/feed.xml".to_string(),
            description: Some("Test description".to_string()),
            category: Some("Technology".to_string()),
            icon: Some("🔥".to_string()),
        };

        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateFeedRequest {
            name: "".to_string(), // Invalid: empty name
            feed_type: "invalid".to_string(), // Invalid: wrong type
            feed_url: "not-a-url".to_string(), // Invalid: not a URL
            description: None,
            category: None,
            icon: None,
        };

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_feed_status() {
        let mut model = Model {
            id: 1,
            name: "Test Feed".to_string(),
            feed_type: "rss".to_string(),
            feed_url: "https://example.com/feed.xml".to_string(),
            description: None,
            category: None,
            status: "loading".to_string(),
            icon: None,
            created_at: Utc::now(),
            last_updated: None,
            summary_count: 0,
        };

        assert!(model.is_loading());
        assert!(!model.is_active());
        assert!(!model.has_error());

        model.set_status(FeedStatus::Active);
        assert!(model.is_active());
        assert!(!model.is_loading());
        assert!(!model.has_error());

        model.increment_summary_count();
        assert_eq!(model.summary_count, 1);

        model.decrement_summary_count();
        assert_eq!(model.summary_count, 0);
    }

    #[test]
    fn test_feed_type_validation() {
        assert!(validate_feed_type("rss").is_ok());
        assert!(validate_feed_type("atom").is_err());
        assert!(validate_feed_type("json").is_err());
        assert!(validate_feed_type("").is_err());
    }
}