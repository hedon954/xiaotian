use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// GitHub API token
    pub github_token: Option<String>,
    /// Default update days to fetch (for fetch command)
    pub default_fetch_days: u32,
    /// Default number of updates to show
    pub default_show_limit: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            github_token: None,
            default_fetch_days: 7,
            default_show_limit: 10,
        }
    }
}

impl AppConfig {
    /// Save config to file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)
    }

    /// Load config from file
    pub fn load_from_file(path: &PathBuf) -> Result<Self, std::io::Error> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Get config file path
    pub fn get_default_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("xiaotian");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }
}
