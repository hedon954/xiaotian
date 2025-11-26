use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub email: EmailConfig,
    pub ai: AiConfig,
    pub sync: SyncConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_request_size: usize,
    pub request_timeout: u64,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
    pub command_timeout: u64,
    pub default_ttl: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_use_tls: bool,
    pub default_sender: String,
    pub default_template: EmailTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub subject: String,
    pub header_text: String,
    pub footer_text: String,
    pub include_original_links: bool,
    pub group_by_feed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub provider: String, // "openai", "anthropic", "local"
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub default_max_items: u32,
    pub default_summary_length: SummaryLength,
    pub sync_interval_minutes: u64,
    pub request_timeout_seconds: u64,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SummaryLength {
    Short,
    Medium,
    Long,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub log_level: String,
    pub max_chat_sessions_per_user: u32,
    pub max_feeds_per_user: u32,
    pub data_retention_days: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            email: EmailConfig::default(),
            ai: AiConfig::default(),
            sync: SyncConfig::default(),
            app: AppConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            max_request_size: env::var("MAX_REQUEST_SIZE")
                .unwrap_or_else(|_| "10485760".to_string())
                .parse()
                .unwrap_or(10 * 1024 * 1024), // 10MB
            request_timeout: env::var("REQUEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://postgres:password@localhost:5432/xiaotian".to_string()
            }),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            connect_timeout: env::var("DB_CONNECT_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            idle_timeout: env::var("DB_IDLE_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .unwrap_or(600),
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            max_connections: env::var("REDIS_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            connection_timeout: env::var("REDIS_CONNECT_TIMEOUT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            command_timeout: env::var("REDIS_COMMAND_TIMEOUT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            default_ttl: env::var("REDIS_DEFAULT_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string()),
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            smtp_username: env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string()),
            smtp_password: env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string()),
            smtp_use_tls: env::var("SMTP_USE_TLS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            default_sender: env::var("EMAIL_SENDER")
                .unwrap_or_else(|_| "小天AI助手 <noreply@xiaotian.ai>".to_string()),
            default_template: EmailTemplate::default(),
        }
    }
}

impl Default for EmailTemplate {
    fn default() -> Self {
        Self {
            subject: env::var("EMAIL_SUBJECT")
                .unwrap_or_else(|_| "每日技术资讯摘要 - {date}".to_string()),
            header_text: env::var("EMAIL_HEADER_TEXT")
                .unwrap_or_else(|_| "以下是今日为您精选的技术资讯摘要：".to_string()),
            footer_text: env::var("EMAIL_FOOTER_TEXT")
                .unwrap_or_else(|_| "感谢使用小天AI助手".to_string()),
            include_original_links: env::var("EMAIL_INCLUDE_ORIGINAL_LINKS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            group_by_feed: env::var("EMAIL_GROUP_BY_FEED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: env::var("AI_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            provider: env::var("AI_PROVIDER").unwrap_or_else(|_| "openai".to_string()),
            api_key: env::var("AI_API_KEY").ok(),
            model: env::var("AI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
            max_tokens: env::var("AI_MAX_TOKENS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000),
            temperature: env::var("AI_TEMPERATURE")
                .unwrap_or_else(|_| "0.7".to_string())
                .parse()
                .unwrap_or(0.7),
            timeout: env::var("AI_TIMEOUT")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            default_max_items: env::var("SYNC_MAX_ITEMS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .unwrap_or(50),
            default_summary_length: match env::var("SYNC_SUMMARY_LENGTH")
                .unwrap_or_else(|_| "medium".to_string())
                .as_str()
            {
                "short" => SummaryLength::Short,
                "long" => SummaryLength::Long,
                _ => SummaryLength::Medium,
            },
            sync_interval_minutes: env::var("SYNC_INTERVAL_MINUTES")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            request_timeout_seconds: env::var("SYNC_REQUEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            user_agent: env::var("SYNC_USER_AGENT")
                .unwrap_or_else(|_| "Xiaotian-RSS-Reader/1.0".to_string()),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: env::var("APP_NAME").unwrap_or_else(|_| "Xiaotian API".to_string()),
            version: env::var("APP_VERSION").unwrap_or_else(|_| "0.1.0".to_string()),
            environment: env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            max_chat_sessions_per_user: env::var("MAX_CHAT_SESSIONS_PER_USER")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            max_feeds_per_user: env::var("MAX_FEEDS_PER_USER")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .unwrap_or(50),
            data_retention_days: env::var("DATA_RETENTION_DAYS")
                .unwrap_or_else(|_| "365".to_string())
                .parse()
                .unwrap_or(365),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Try to load from .env file if it exists
        dotenvy::dotenv().ok();

        let config = Config::default();

        // Validate required configuration
        if config.database.url.is_empty() {
            return Err("DATABASE_URL is required".into());
        }

        if config.email.smtp_host.is_empty() || config.email.smtp_username.is_empty() {
            tracing::warn!("Email configuration is incomplete. Email features may not work.");
        }

        if config.ai.enabled && config.ai.api_key.is_none() {
            tracing::warn!("AI features are enabled but API key is not configured.");
        }

        Ok(config)
    }

    pub fn is_development(&self) -> bool {
        self.app.environment == "development"
    }

    pub fn is_production(&self) -> bool {
        self.app.environment == "production"
    }
}