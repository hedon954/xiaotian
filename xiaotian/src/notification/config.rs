use std::sync::Arc;
use tracing::info;

use crate::config::{AppConfig, NotificationConfig};

use super::{EmailNotifier, NotificationError, NotificationMessage, Notifier};

/// Notification manager
pub struct NotificationManager {
    pub config: NotificationConfig,
    email_notifier: Option<Arc<EmailNotifier>>,
}

impl NotificationManager {
    /// Create notification manager from app config
    pub fn from_config(config: &AppConfig) -> Self {
        let mut manager = Self {
            config: config.notification.clone(),
            email_notifier: None,
        };

        // initialize email notifier
        if let Some(email_config) = &config.notification.email {
            let email_nofitier = EmailNotifier::new(email_config.clone());
            manager.email_notifier = Some(Arc::new(email_nofitier));
            info!("Email notifier initialized successfully");
        }

        manager
    }

    /// Send notification
    pub async fn send(
        &self,
        message: &NotificationMessage,
        to_emails: Vec<String>,
    ) -> Result<(), NotificationError> {
        if !self.config.enabled {
            info!("Notification is disabled, skipping");
            return Ok(());
        }

        match self.config.default_channel.as_str() {
            "email" => self.send_email(message, to_emails).await,
            _ => Err(NotificationError::Config(format!(
                "Unknown notification channel: {}",
                self.config.default_channel
            ))),
        }
    }

    /// Send email notification
    async fn send_email(
        &self,
        message: &NotificationMessage,
        to_emails: Vec<String>,
    ) -> Result<(), NotificationError> {
        if let Some(notifier) = &self.email_notifier {
            notifier.send(message, to_emails).await
        } else {
            Err(NotificationError::ChannelUnavailable(
                "Email notifier is not available".to_string(),
            ))
        }
    }
}
