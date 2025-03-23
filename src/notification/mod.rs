use async_trait::async_trait;
use std::collections::HashMap;
use thiserror::Error;

/// 通知系统错误类型
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

/// 通知消息
#[derive(Debug, Clone)]
pub struct NotificationMessage {
    /// 通知主题
    pub subject: String,
    /// 通知内容
    pub content: String,
    /// 元数据，用于存储额外信息
    pub metadata: HashMap<String, String>,
}

impl NotificationMessage {
    /// 创建新的通知消息
    pub fn new(subject: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            content: content.into(),
            metadata: HashMap::new(),
        }
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// 通知发送器特征
#[async_trait]
pub trait Notifier: Send + Sync {
    /// 发送通知
    async fn send(&self, message: &NotificationMessage) -> Result<(), NotificationError>;
}

mod config;
mod email;

pub use config::NotificationManager;
pub use email::{EmailConfig, EmailNotifier};
