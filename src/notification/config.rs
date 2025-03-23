use std::sync::Arc;
use tracing::info;

use crate::config::{AppConfig, NotificationConfig};

use super::{EmailNotifier, NotificationError, NotificationMessage, Notifier};

/// 通知管理器
pub struct NotificationManager {
    config: NotificationConfig,
    email_notifier: Option<Arc<EmailNotifier>>,
}

impl NotificationManager {
    /// 从应用配置创建通知管理器
    pub fn from_config(config: &AppConfig) -> Self {
        let mut manager = Self {
            config: config.notification.clone(),
            email_notifier: None,
        };

        // 初始化邮件通知器
        if let Some(email_config) = &config.notification.email {
            let email_nofitier = EmailNotifier::new(email_config.clone());
            manager.email_notifier = Some(Arc::new(email_nofitier));
            info!("Email notifier initialized successfully");
        }

        manager
    }

    /// 发送通知
    pub async fn send(&self, message: &NotificationMessage) -> Result<(), NotificationError> {
        if !self.config.enabled {
            info!("Notification is disabled, skipping");
            return Ok(());
        }

        match self.config.default_channel.as_str() {
            "email" => self.send_email(message).await,
            _ => Err(NotificationError::Config(format!(
                "Unknown notification channel: {}",
                self.config.default_channel
            ))),
        }
    }

    /// 发送邮件通知
    async fn send_email(&self, message: &NotificationMessage) -> Result<(), NotificationError> {
        if let Some(notifier) = &self.email_notifier {
            notifier.send(message).await
        } else {
            Err(NotificationError::ChannelUnavailable(
                "Email notifier is not available".to_string(),
            ))
        }
    }
}
