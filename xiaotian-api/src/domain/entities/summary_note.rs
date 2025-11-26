use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "summary_notes")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "Text", not_null)]
    pub content: String,

    #[sea_orm(column_type = "Integer", not_null)]
    pub summary_id: i32,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTime<Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTime<Utc>,
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

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
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
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_summary_note_model() {
        let mut note = Model {
            id: 1,
            content: "This is a test note for a summary.".to_string(),
            summary_id: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let original_updated = note.updated_at;
        note.update_timestamp();
        assert!(note.updated_at > original_updated);

        let preview = note.content_preview(20);
        assert_eq!(preview, "This is a test note...");
        assert_eq!(note.content_preview(100), note.content);
    }

    #[test]
    fn test_create_summary_note_request_validation() {
        let valid_request = CreateSummaryNoteRequest {
            content: "This is a valid note content.".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let empty_request = CreateSummaryNoteRequest {
            content: "".to_string(), // Invalid: empty content
        };
        assert!(empty_request.validate().is_err());

        let too_long_request = CreateSummaryNoteRequest {
            content: "a".repeat(2001), // Invalid: too long
        };
        assert!(too_long_request.validate().is_err());
    }

    #[test]
    fn test_update_summary_note_request_validation() {
        let valid_request = UpdateSummaryNoteRequest {
            content: "Updated note content.".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateSummaryNoteRequest {
            content: "".to_string(), // Invalid: empty content
        };
        assert!(invalid_request.validate().is_err());
    }
}