use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Feed URL invalid: {0}")]
    FeedUrlInvalid(String),

    #[error("Feed already exists: {0}")]
    FeedAlreadyExists(String),

    #[error("Cron expression invalid: {0}")]
    CronExpressionInvalid(String),

    #[error("Email configuration invalid")]
    EmailConfigInvalid,

    #[error("Sync already in progress")]
    SyncInProgress,

    #[error("Task not enabled")]
    TaskNotEnabled,

    #[error("Chat session limit exceeded")]
    ChatSessionLimitExceeded,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("RSS parsing error: {0}")]
    RssParsingError(String),

    #[error("AI service error: {0}")]
    AiServiceError(String),

    #[error("Email send error: {0}")]
    EmailSendError(String),
}

impl AppError {
    pub fn error_code(&self) -> u32 {
        match self {
            AppError::ValidationError(_) => 1001,
            AppError::Unauthorized(_) => 1002,
            AppError::Forbidden(_) => 1003,
            AppError::NotFound { .. } => 1004,
            AppError::Conflict(_) => 1006,
            AppError::RateLimitExceeded => 1007,
            AppError::InternalServerError(_) => 1008,
            AppError::ServiceUnavailable(_) => 1009,

            // Business error codes
            AppError::FeedUrlInvalid(_) => 2001,
            AppError::FeedAlreadyExists(_) => 2002,
            AppError::CronExpressionInvalid(_) => 2003,
            AppError::EmailConfigInvalid => 2004,
            AppError::SyncInProgress => 2005,
            AppError::TaskNotEnabled => 2006,
            AppError::ChatSessionLimitExceeded => 2007,

            // Other errors
            _ => 5000,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,

            // Business logic errors
            AppError::FeedUrlInvalid(_)
            | AppError::FeedAlreadyExists(_)
            | AppError::CronExpressionInvalid(_)
            | AppError::EmailConfigInvalid
            | AppError::SyncInProgress
            | AppError::TaskNotEnabled
            | AppError::ChatSessionLimitExceeded => StatusCode::BAD_REQUEST,

            // Network and external service errors
            AppError::NetworkError(_) => StatusCode::BAD_GATEWAY,
            AppError::RssParsingError(_) => StatusCode::BAD_GATEWAY,
            AppError::AiServiceError(_) => StatusCode::BAD_GATEWAY,
            AppError::EmailSendError(_) => StatusCode::BAD_GATEWAY,

            // Database errors
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(validation_errors) => {
                let details: Vec<String> = validation_errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            format!("{}: {}", field, error.message.as_ref().unwrap_or(&"Invalid value".into()))
                        })
                    })
                    .collect();

                (StatusCode::BAD_REQUEST, format!("Validation failed: {}", details.join(", ")))
            }
            AppError::NotFound { entity, id } => {
                (StatusCode::NOT_FOUND, format!("{} with id '{}' not found", entity, id))
            }
            _ => (self.status_code(), self.to_string()),
        };

        let body = Json(json!({
            "code": self.error_code(),
            "data": null,
            "message": error_message,
            "trace_id": uuid::Uuid::new_v4().to_string(),
            "request_id": uuid::Uuid::new_v4().to_string()
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;