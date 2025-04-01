mod config;
mod email;

use async_trait::async_trait;
use std::collections::HashMap;
use thiserror::Error;

pub use config::NotificationManager;
pub use email::{EmailConfig, EmailNotifier};

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("Config error: {0}")]
    Config(String),
    #[error("send failed: {0}")]
    SendFailed(String),
    #[error("Invalid template: {0}")]
    Template(String),
    #[error("Unavailable channel: {0}")]
    ChannelUnavailable(String),
}

/// The notification message
#[derive(Debug, Clone)]
pub struct NotificationMessage {
    /// The subject of the notification
    pub subject: String,
    /// The content of the notification
    pub content: String,
    /// The metadata, for storing additional information
    pub metadata: HashMap<String, String>,
}

impl NotificationMessage {
    /// Create a new notification message
    pub fn new(subject: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            content: content.into(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// The notification sender trait
#[async_trait]
pub trait Notifier: Send + Sync {
    /// Send notification
    async fn send(
        &self,
        message: &NotificationMessage,
        to: Vec<String>,
    ) -> Result<(), NotificationError>;
}
