use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::notification::EmailConfig;

/// Application configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    /// GitHub configuration
    pub github: GitHubConfig,
    /// 通知系统配置
    #[serde(default)]
    pub notification: NotificationConfig,
}

/// GitHub API configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHubConfig {
    /// GitHub API token
    pub token: Option<String>,
    /// GitHub API URL (for enterprise GitHub instances)
    pub api_url: Option<String>,
}

/// Notification system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// whether to enable notifications
    #[serde(default = "default_notification_enabled")]
    pub enabled: bool,
    /// default notification channel
    #[serde(default = "default_notification_channel")]
    pub default_channel: String,
    /// email notification configuration
    pub email: Option<EmailConfig>,
}

fn default_notification_enabled() -> bool {
    false
}

fn default_notification_channel() -> String {
    "email".to_string()
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: default_notification_enabled(),
            default_channel: default_notification_channel(),
            email: None,
        }
    }
}

impl AppConfig {
    /// Load config from file
    pub fn load(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();
        if !path.exists() {
            anyhow::bail!("Config file not found");
        }

        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Get GitHub token
    pub fn github_token(&self) -> Option<String> {
        self.github.token.clone()
    }
}
