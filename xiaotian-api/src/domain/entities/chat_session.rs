use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(200))", not_null)]
    pub title: String,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTime<Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chat_message::Entity")]
    Messages,
}

impl Related<super::chat_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
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
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_messages")]
pub struct MessageModel {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub message_id: String, // UUID format like "msg-{uuid}"

    #[sea_orm(column_type = "Integer", not_null)]
    pub session_id: i32,

    #[sea_orm(column_type = "String(Some(20))", not_null)]
    pub message_type: String, // "user" or "assistant"

    #[sea_orm(column_type = "Text", not_null)]
    pub content: String,

    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub sources: Option<serde_json::Value>, // Array of sources for assistant messages

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub timestamp: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum MessageRelation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::SessionId",
        to = "super::Column::Id"
    )]
    Session,
}

impl Related<Entity> for MessageEntity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for MessageActiveModel {
    fn new() -> Self {
        let message_id = format!("msg-{}", Uuid::new_v4());
        Self {
            message_id: Set(message_id),
            timestamp: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
}

impl MessageModel {
    pub fn message_type(&self) -> MessageType {
        match self.message_type.as_str() {
            "user" => MessageType::User,
            "assistant" => MessageType::Assistant,
            _ => MessageType::User, // Default to User for unknown types
        }
    }

    pub fn is_user(&self) -> bool {
        self.message_type() == MessageType::User
    }

    pub fn is_assistant(&self) -> bool {
        self.message_type() == MessageType::Assistant
    }

    pub fn sources(&self) -> Vec<ChatMessageSource> {
        match &self.sources {
            Some(sources_json) => {
                serde_json::from_value(sources_json.clone()).unwrap_or_default()
            }
            None => Vec::new(),
        }
    }

    pub fn set_sources(&mut self, sources: Vec<ChatMessageSource>) {
        self.sources = Some(serde_json::to_value(sources).unwrap_or(serde_json::Value::Null));
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    User,
    Assistant,
}

impl ToString for MessageType {
    fn to_string(&self) -> String {
        match self {
            MessageType::User => "user".to_string(),
            MessageType::Assistant => "assistant".to_string(),
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateChatSessionRequest {
    #[validate(length(min = 1, max = 100, message = "Title must be between 1 and 100 characters"))]
    pub title: Option<String>,

    #[validate(length(min = 1, max = 2000, message = "Initial message must be between 1 and 2000 characters"))]
    pub initial_message: Option<String>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct SendMessageRequest {
    #[validate(length(min = 1, max = 2000, message = "Content must be between 1 and 2000 characters"))]
    pub content: String,

    pub context: Option<SendMessageContext>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct SendMessageContext {
    pub summary_id: Option<String>,
    pub feed_id: Option<String>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct UpdateChatSessionRequest {
    #[validate(length(min = 1, max = 100, message = "Title must be between 1 and 100 characters"))]
    pub title: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct ChatSessionListParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageSource {
    pub summary_id: i32,
    pub summary_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub user_message: ChatMessage,
    pub assistant_message: ChatMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub message_type: String,
    pub content: String,
    pub sources: Option<Vec<ChatMessageSource>>,
    pub timestamp: DateTime<Utc>,
}

impl From<MessageModel> for ChatMessage {
    fn from(model: MessageModel) -> Self {
        Self {
            id: model.message_id,
            message_type: model.message_type,
            content: model.content,
            sources: if model.sources.is_some() {
                Some(model.sources())
            } else {
                None
            },
            timestamp: model.timestamp,
        }
    }
}

impl Model {
    pub fn message_count(&self, messages: &[MessageModel]) -> i32 {
        messages.iter()
            .filter(|msg| msg.session_id == self.id)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_message_type() {
        let message = MessageModel {
            id: 1,
            message_id: "msg-123".to_string(),
            session_id: 1,
            message_type: "user".to_string(),
            content: "Hello".to_string(),
            sources: None,
            timestamp: Utc::now(),
        };

        assert!(message.is_user());
        assert!(!message.is_assistant());
        assert_eq!(message.message_type(), MessageType::User);
    }

    #[test]
    fn test_message_sources() {
        let mut message = MessageModel {
            id: 1,
            message_id: "msg-123".to_string(),
            session_id: 1,
            message_type: "assistant".to_string(),
            content: "Here's the answer".to_string(),
            sources: None,
            timestamp: Utc::now(),
        };

        let sources = vec![
            ChatMessageSource {
                summary_id: 1,
                summary_title: "Test Summary".to_string(),
            }
        ];

        message.set_sources(sources.clone());
        assert_eq!(message.sources(), sources);
    }

    #[test]
    fn test_create_chat_session_request_validation() {
        let valid_request = CreateChatSessionRequest {
            title: Some("Test Session".to_string()),
            initial_message: Some("Hello, I need help".to_string()),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_title = CreateChatSessionRequest {
            title: Some("".to_string()), // Invalid: empty title
            initial_message: None,
        };
        assert!(invalid_title.validate().is_err());

        let invalid_message = CreateChatSessionRequest {
            title: None,
            initial_message: Some("a".repeat(2001)), // Invalid: too long
        };
        assert!(invalid_message.validate().is_err());
    }

    #[test]
    fn test_send_message_request_validation() {
        let valid_request = SendMessageRequest {
            content: "What is the latest news?".to_string(),
            context: Some(SendMessageContext {
                summary_id: Some("123".to_string()),
                feed_id: None,
            }),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = SendMessageRequest {
            content: "".to_string(), // Invalid: empty content
            context: None,
        };
        assert!(invalid_request.validate().is_err());

        let too_long_request = SendMessageRequest {
            content: "a".repeat(2001), // Invalid: too long
            context: None,
        };
        assert!(too_long_request.validate().is_err());
    }

    #[test]
    fn test_chat_session_timestamps() {
        let mut session = Model {
            id: 1,
            title: "Test Session".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let original_updated = session.updated_at;
        session.update_timestamp();
        assert!(session.updated_at > original_updated);
    }

    #[test]
    fn test_message_id_generation() {
        let message_active_model = MessageActiveModel::new();
        if let Set(message_id) = message_active_model.message_id {
            assert!(message_id.starts_with("msg-"));
            assert!(Uuid::parse_str(&message_id[4..]).is_ok());
        } else {
            panic!("Message ID should be set");
        }
    }
}