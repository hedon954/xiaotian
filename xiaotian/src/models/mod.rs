pub mod hackernews;
pub mod repository;
pub mod update;

use core::fmt;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use enum_dispatch::enum_dispatch;
pub use hackernews::{HackerNews, HackerNewsFeedType};
pub use repository::Repository;
use serde::{Deserialize, Serialize};
pub use update::{Update, UpdateEventType};

use crate::error::AppError;

#[async_trait]
#[enum_dispatch]
pub trait Fetcher {
    /// Fetch updates from the source since the given time
    async fn fetch_updates(&self, since: DateTime<Utc>) -> Result<String, AppError>;
}

#[enum_dispatch(Fetcher)]
pub enum Source {
    GitHub(Repository),
    HackerNews(HackerNews),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum SourceType {
    GitHub,
    HackerNews,
}

impl TryFrom<String> for SourceType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "github" => Ok(SourceType::GitHub),
            "hackernews" | "hacker_news" | "hacker-news" | "hn" => Ok(SourceType::HackerNews),
            _ => Err(format!("Invalid source type: {}", value)),
        }
    }
}

impl TryFrom<i8> for SourceType {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SourceType::GitHub),
            2 => Ok(SourceType::HackerNews),
            _ => Err(format!("Invalid source type: {}", value)),
        }
    }
}

impl From<SourceType> for i8 {
    fn from(value: SourceType) -> Self {
        match value {
            SourceType::GitHub => 1,
            SourceType::HackerNews => 2,
        }
    }
}

impl Source {
    pub fn get_name(&self) -> String {
        match self {
            Source::GitHub(repo) => format!("Github: {}", repo.full_name()),
            Source::HackerNews(hn) => format!("HackerNews: {}", hn.feed_type),
        }
    }

    pub fn get_type(&self) -> SourceType {
        match self {
            Source::GitHub(_) => SourceType::GitHub,
            Source::HackerNews(_) => SourceType::HackerNews,
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
            SourceType::HackerNews => write!(f, "HackerNews"),
        }
    }
}
