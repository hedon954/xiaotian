pub mod config;
pub mod domain;
pub mod shared;

// Re-export commonly used types
pub use config::{Config, SummaryLength, EmailConfig};
pub use shared::{AppError, AppResult, ApiResponse, PaginationData, PaginationParams};