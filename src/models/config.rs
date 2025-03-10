use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    /// GitHub configuration
    pub github: GitHubConfig,
}

/// GitHub API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// GitHub API token
    pub token: Option<String>,
    /// GitHub API URL (for enterprise GitHub instances)
    pub api_url: Option<String>,
    /// GitHub API request timeout (in seconds)
    pub timeout_seconds: u64,
    /// GitHub API request retries
    pub max_retries: u32,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            token: None,
            api_url: None,
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}

impl AppConfig {
    /// Load config from file
    pub fn load_from_file(path: &PathBuf) -> anyhow::Result<Self> {
        if !path.exists() {
            anyhow::bail!("Config file not found");
        }

        let contents = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Load from default config file
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::get_default_path();
        Self::load_from_file(&path)
    }

    /// Get config file path
    pub fn get_default_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("xiaotian");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    /// Get GitHub token
    pub fn github_token(&self) -> Option<String> {
        self.github.token.clone()
    }
}
