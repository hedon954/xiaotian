use axum::{extract::Request, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: T,
    pub message: String,
    pub trace_id: String,
    pub request_id: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            code: 0,
            data,
            message: message.to_string(),
            trace_id: Uuid::new_v4().to_string(),
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn error(code: i32, message: &str) -> ApiResponse<Value> {
        ApiResponse {
            code,
            data: Value::Null,
            message: message.to_string(),
            trace_id: Uuid::new_v4().to_string(),
            request_id: Uuid::new_v4().to_string(),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
        }
    }
}

impl PaginationParams {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(20).clamp(1, 100)
    }

    pub fn offset(&self) -> u64 {
        ((self.page() - 1) * self.page_size()) as u64
    }

    pub fn limit(&self) -> u64 {
        self.page_size() as u64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationData<T> {
    pub items: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
    pub total_pages: u32,
}

impl<T> PaginationData<T> {
    pub fn new(items: Vec<T>, total: u64, params: &PaginationParams) -> Self {
        let page = params.page();
        let page_size = params.page_size();
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        Self {
            items,
            pagination: PaginationMeta {
                page,
                page_size,
                total,
                total_pages: total_pages.max(1),
            },
        }
    }
}

// Middleware to add request/response IDs
#[derive(Debug, Clone)]
pub struct ResponseHeaders;

impl ResponseHeaders {
    pub fn add_request_headers() {
        // This would be implemented as middleware
    }
}