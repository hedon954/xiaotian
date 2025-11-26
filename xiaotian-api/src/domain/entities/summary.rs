use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "summaries")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(500))", not_null)]
    pub title: String,

    #[sea_orm(column_type = "Text", not_null)]
    pub content: String,

    #[sea_orm(column_type = "Text", not_null)]
    pub full_content: String,

    #[sea_orm(column_type = "String(Some(1000))", not_null)]
    pub original_url: String,

    #[sea_orm(column_type = "String(Some(50))", not_null)]
    pub published_at: String, // Chinese date format like "2025年7月8日"

    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub tags: Option<serde_json::Value>, // Array of strings

    #[sea_orm(column_type = "Integer", not_null)]
    pub feed_id: i32,

    #[sea_orm(column_type = "String(Some(200))", not_null)]
    pub feed_name: String,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub note_count: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub source_material_count: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub related_summary_count: i32,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTime<Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::feed::Entity",
        from = "Column::FeedId",
        to = "super::feed::Column::Id"
    )]
    Feed,

    #[sea_orm(has_many = "super::summary_note::Entity")]
    Notes,

    #[sea_orm(has_many = "super::source_material::Entity")]
    SourceMaterials,

    #[sea_orm(has_many = "super::related_summary::Entity")]
    RelatedSummaries,

    #[sea_orm(has_many = "super::related_summary::Entity")]
    ReverseRelatedSummaries,
}

impl Related<super::feed::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Feed.def()
    }
}

impl Related<super::summary_note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notes.def()
    }
}

impl Related<super::source_material::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SourceMaterials.def()
    }
}

impl Related<super::related_summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelatedSummaries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            note_count: Set(0),
            source_material_count: Set(0),
            related_summary_count: Set(0),
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
    pub fn tags(&self) -> Vec<String> {
        match &self.tags {
            Some(tags_json) => {
                serde_json::from_value(tags_json.clone()).unwrap_or_default()
            }
            None => Vec::new(),
        }
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = Some(serde_json::to_value(tags).unwrap_or(serde_json::Value::Null));
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        let mut tags = self.tags();
        if !tags.contains(&tag) && !tag.trim().is_empty() {
            tags.push(tag.trim());
            self.set_tags(tags);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        let mut tags = self.tags();
        tags.retain(|t| t != tag);
        self.set_tags(tags);
    }

    pub fn increment_note_count(&mut self) {
        self.note_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_note_count(&mut self) {
        if self.note_count > 0 {
            self.note_count -= 1;
            self.updated_at = Utc::now();
        }
    }

    pub fn increment_source_material_count(&mut self) {
        self.source_material_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_source_material_count(&mut self) {
        if self.source_material_count > 0 {
            self.source_material_count -= 1;
            self.updated_at = Utc::now();
        }
    }

    pub fn increment_related_summary_count(&mut self) {
        self.related_summary_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_related_summary_count(&mut self) {
        if self.related_summary_count > 0 {
            self.related_summary_count -= 1;
            self.updated_at = Utc::now();
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(&tag.to_string())
    }

    pub fn content_preview(&self, max_length: usize) -> String {
        let content = self.content.trim();
        if content.len() <= max_length {
            content.to_string()
        } else {
            format!("{}...", &content[..max_length])
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateSummaryNoteRequest {
    #[validate(length(min = 1, max = 2000, message = "Content must be between 1 and 2000 characters"))]
    pub content: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct UpdateSummaryNoteRequest {
    #[validate(length(min = 1, max = 2000, message = "Content must be between 1 and 2000 characters"))]
    pub content: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct AddTagRequest {
    #[validate(length(min = 1, max = 50, message = "Tag must be between 1 and 50 characters"))]
    pub tag: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateSummaryRequest {
    #[validate(length(min = 1, max = 500, message = "Title must be between 1 and 500 characters"))]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,

    #[validate(length(min = 1, message = "Full content cannot be empty"))]
    pub full_content: String,

    #[validate(url(message = "Original URL must be a valid URL"))]
    pub original_url: String,

    #[validate(length(min = 1, max = 50, message = "Published date must be between 1 and 50 characters"))]
    pub published_at: String,

    pub tags: Option<Vec<String>>,

    #[validate(range(min = 1, message = "Feed ID must be a positive integer"))]
    pub feed_id: i32,

    #[validate(length(min = 1, max = 200, message = "Feed name must be between 1 and 200 characters"))]
    pub feed_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryListParams {
    pub feed_id: Option<i32>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub tags: Option<String>, // Comma-separated tags
    pub search: Option<String>, // Search in title and content
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedSummaryParams {
    pub limit: Option<u32>,
    pub min_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedSummary {
    pub id: i32,
    pub title: String,
    pub relevance_score: f64,
    pub relation_type: String, // "content", "temporal", "source", "tag"
    pub shared_tags: Option<Vec<String>>,
    pub published_at: String,
    pub excerpt: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_summary_tags() {
        let mut summary = Model {
            id: 1,
            title: "Test Summary".to_string(),
            content: "Test content".to_string(),
            full_content: "Test full content".to_string(),
            original_url: "https://example.com".to_string(),
            published_at: "2025年1月8日".to_string(),
            tags: Some(serde_json::to_value(vec!["rust", "programming"]).unwrap()),
            feed_id: 1,
            feed_name: "Test Feed".to_string(),
            note_count: 0,
            source_material_count: 0,
            related_summary_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let tags = summary.tags();
        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&"rust".to_string()));
        assert!(tags.contains(&"programming".to_string()));

        summary.add_tag("ai".to_string());
        let tags = summary.tags();
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"ai".to_string()));

        summary.remove_tag("rust");
        let tags = summary.tags();
        assert_eq!(tags.len(), 2);
        assert!(!tags.contains(&"rust".to_string()));

        assert!(summary.has_tag("programming"));
        assert!(!summary.has_tag("rust"));
    }

    #[test]
    fn test_content_preview() {
        let summary = Model {
            id: 1,
            title: "Test Summary".to_string(),
            content: "This is a long content that should be truncated when it exceeds the maximum length specified.".to_string(),
            full_content: "Full content".to_string(),
            original_url: "https://example.com".to_string(),
            published_at: "2025年1月8日".to_string(),
            tags: None,
            feed_id: 1,
            feed_name: "Test Feed".to_string(),
            note_count: 0,
            source_material_count: 0,
            related_summary_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let preview = summary.content_preview(50);
        assert_eq!(preview.len(), 53); // 50 chars + "..."
        assert!(preview.ends_with("..."));

        let short_preview = summary.content_preview(100);
        assert_eq!(short_preview, summary.content);
    }

    #[test]
    fn test_create_summary_note_request_validation() {
        let valid_request = CreateSummaryNoteRequest {
            content: "This is a valid note".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateSummaryNoteRequest {
            content: "".to_string(), // Invalid: empty content
        };
        assert!(invalid_request.validate().is_err());

        let too_long_request = CreateSummaryNoteRequest {
            content: "a".repeat(2001), // Invalid: too long
        };
        assert!(too_long_request.validate().is_err());
    }

    #[test]
    fn test_summary_counters() {
        let mut summary = Model {
            id: 1,
            title: "Test Summary".to_string(),
            content: "Test content".to_string(),
            full_content: "Test full content".to_string(),
            original_url: "https://example.com".to_string(),
            published_at: "2025年1月8日".to_string(),
            tags: None,
            feed_id: 1,
            feed_name: "Test Feed".to_string(),
            note_count: 0,
            source_material_count: 0,
            related_summary_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        summary.increment_note_count();
        assert_eq!(summary.note_count, 1);

        summary.increment_source_material_count();
        assert_eq!(summary.source_material_count, 1);

        summary.increment_related_summary_count();
        assert_eq!(summary.related_summary_count, 1);

        summary.decrement_note_count();
        assert_eq!(summary.note_count, 0);

        // Should not go below zero
        summary.decrement_note_count();
        assert_eq!(summary.note_count, 0);
    }
}