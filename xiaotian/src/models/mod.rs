pub mod repository;
pub mod update;

use core::fmt;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use enum_dispatch::enum_dispatch;
pub use repository::Repository;
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
pub enum Source {
    GitHub(Repository),
}

#[derive(Debug)]
pub enum SourceType {
    GitHub,
}

impl TryFrom<String> for SourceType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "github" => Ok(SourceType::GitHub),
            _ => Err(format!("Invalid source type: {}", value)),
        }
    }
}

impl TryFrom<i8> for SourceType {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SourceType::GitHub),
            _ => Err(format!("Invalid source type: {}", value)),
        }
    }
}

impl From<SourceType> for i8 {
    fn from(value: SourceType) -> Self {
        match value {
            SourceType::GitHub => 1,
        }
    }
}

impl Source {
    pub fn get_name(&self) -> String {
        match self {
            Source::GitHub(repo) => format!("Github: {}", repo.full_name()),
        }
    }

    pub fn get_type(&self) -> SourceType {
        match self {
            Source::GitHub(_) => SourceType::GitHub,
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.get_type(), self.get_name())
    }
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceType::GitHub => write!(f, "GitHub"),
        }
    }
}
