use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "source_materials")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "Integer", not_null)]
    pub summary_id: i32,

    #[sea_orm(column_type = "String(Some(500))", not_null)]
    pub title: String,

    #[sea_orm(column_type = "String(Some(1000))", not_null)]
    pub url: String,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub published_at: DateTime<Utc>,

    #[sea_orm(column_type = "String(Some(200))", nullable)]
    pub author: Option<String>,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub source: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub excerpt: Option<String>,

    #[sea_orm(column_type = "Integer", nullable)]
    pub word_count: Option<i32>,

    #[sea_orm(column_type = "Integer", nullable)]
    pub reading_time: Option<i32>, // in minutes

    #[sea_orm(column_type = "String(Some(20))", not_null, default_value = "\"article\"")]
    pub language: String, // "en", "zh", etc.

    #[sea_orm(column_type = "String(Some(20))", not_null, default_value = "\"article\"")]
    pub content_type: String, // "article", "video", "podcast", "document"
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::summary::Entity",
        from = "Column::SummaryId",
        to = "super::summary::Column::Id"
    )]
    Summary,
}

impl Related<super::summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Summary.def()
    }
}

impl Model {
    pub fn content_type(&self) -> ContentType {
        match self.content_type.as_str() {
            "video" => ContentType::Video,
            "podcast" => ContentType::Podcast,
            "document" => ContentType::Document,
            _ => ContentType::Article,
        }
    }

    pub fn estimated_reading_time(&self) -> i32 {
        self.reading_time.unwrap_or_else(|| {
            let words = self.word_count.unwrap_or(500);
            (words / 200).max(1) // Assume 200 words per minute
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    Article,
    Video,
    Podcast,
    Document,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Article => "article".to_string(),
            ContentType::Video => "video".to_string(),
            ContentType::Podcast => "podcast".to_string(),
            ContentType::Document => "document".to_string(),
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateSourceMaterialRequest {
    #[validate(length(min = 1, max = 500, message = "Title must be between 1 and 500 characters"))]
    pub title: String,

    #[validate(url(message = "URL must be a valid URL"))]
    pub url: String,

    pub published_at: DateTime<Utc>,

    #[validate(length(max = 200, message = "Author must be at most 200 characters"))]
    pub author: Option<String>,

    #[validate(length(min = 1, max = 100, message = "Source must be between 1 and 100 characters"))]
    pub source: String,

    #[validate(length(max = 1000, message = "Excerpt must be at most 1000 characters"))]
    pub excerpt: Option<String>,

    #[validate(range(min = 1, message = "Word count must be a positive integer"))]
    pub word_count: Option<i32>,

    pub reading_time: Option<i32>,

    #[validate(length(min = 2, max = 10, message = "Language must be between 2 and 10 characters"))]
    pub language: Option<String>,

    #[validate(custom = "validate_content_type")]
    pub content_type: String,
}

fn validate_content_type(content_type: &str) -> Result<(), validator::ValidationError> {
    if !matches!(content_type, "article" | "video" | "podcast" | "document") {
        return Err(validator::ValidationError::new("invalid_content_type"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_source_material_model() {
        let material = Model {
            id: 1,
            summary_id: 1,
            title: "Test Article".to_string(),
            url: "https://example.com/article".to_string(),
            published_at: Utc::now(),
            author: Some("Test Author".to_string()),
            source: "Tech Blog".to_string(),
            excerpt: Some("Test excerpt".to_string()),
            word_count: Some(1000),
            reading_time: Some(5),
            language: "en".to_string(),
            content_type: "article".to_string(),
        };

        assert_eq!(material.content_type(), ContentType::Article);
        assert_eq!(material.estimated_reading_time(), 5);
    }

    #[test]
    fn test_content_type() {
        assert_eq!(ContentType::Article.to_string(), "article");
        assert_eq!(ContentType::Video.to_string(), "video");
        assert_eq!(ContentType::Podcast.to_string(), "podcast");
        assert_eq!(ContentType::Document.to_string(), "document");
    }

    #[test]
    fn test_create_source_material_request_validation() {
        let valid_request = CreateSourceMaterialRequest {
            title: "Test Article".to_string(),
            url: "https://example.com/article".to_string(),
            published_at: Utc::now(),
            author: Some("Test Author".to_string()),
            source: "Tech Blog".to_string(),
            excerpt: Some("Test excerpt".to_string()),
            word_count: Some(1000),
            reading_time: Some(5),
            language: Some("en".to_string()),
            content_type: "article".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateSourceMaterialRequest {
            title: "".to_string(), // Invalid: empty title
            url: "not-a-url".to_string(), // Invalid: not a URL
            published_at: Utc::now(),
            author: None,
            source: "".to_string(), // Invalid: empty source
            excerpt: None,
            word_count: Some(-1), // Invalid: negative word count
            reading_time: None,
            language: None,
            content_type: "invalid".to_string(), // Invalid: unknown type
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_estimated_reading_time() {
        let mut material = Model {
            id: 1,
            summary_id: 1,
            title: "Test Article".to_string(),
            url: "https://example.com/article".to_string(),
            published_at: Utc::now(),
            author: None,
            source: "Tech Blog".to_string(),
            excerpt: None,
            word_count: None,
            reading_time: None,
            language: "en".to_string(),
            content_type: "article".to_string(),
        };

        // Should use default estimation when reading_time is None
        assert_eq!(material.estimated_reading_time(), 3); // 500 words / 200 words per minute, rounded up

        material.word_count = Some(1000);
        assert_eq!(material.estimated_reading_time(), 5);

        material.reading_time = Some(8);
        assert_eq!(material.estimated_reading_time(), 8); // Should use explicit reading_time
    }
}