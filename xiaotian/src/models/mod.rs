pub mod repository;
pub mod source;
pub mod update;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use enum_dispatch::enum_dispatch;
pub use repository::Repository;
pub use source::{Source, SourceConfig, SourceError, SourceFactory, SourceType};
pub use update::{Update, UpdateEventType};

use crate::error::AppError;

#[async_trait]
#[enum_dispatch]
pub trait Fetcher {
    /// Fetch updates from the source between the start and end time
    async fn fetch_updates(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> Result<String, AppError>;
}

#[enum_dispatch(Fetcher)]
pub enum SourceV2 {
    GitHub(Repository),
}
