use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{FromRow, MySql, MySqlPool, Pool};
use std::time::Duration;
use tracing::{debug, error, info};

use super::error::StorageError;
use super::{HackerNewsStorage, RepositoryStorage, Storage};
use crate::models::{HackerNews, HackerNewsFeedType, Repository, SourceType};

const DEFAULT_MAX_CONNECTIONS: u32 = 5;
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

/// Source data from the database
#[derive(Debug, Clone, FromRow)]
#[allow(unused)]
struct SourceRow {
    id: i32,
    #[sqlx(rename = "type")]
    source_type: String,
    meta: serde_json::Value,
}

/// MySQL implementation of Storage
#[derive(Clone, Debug)]
pub struct MySQLStorage {
    /// The MySQL connection pool
    pool: Pool<MySql>,
}

/// Configuration options for MySQL storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySQLConfig {
    /// Connection URL (mysql://username:password@host:port/database)
    pub dsn: String,
    /// Maximum number of connections in the pool
    pub max_connections: Option<u32>,
    /// Connection timeout in seconds
    pub timeout_seconds: Option<u64>,
}

impl Default for MySQLConfig {
    fn default() -> Self {
        Self {
            dsn: "mysql://root:root@localhost/xiaotian".to_string(),
            max_connections: None,
            timeout_seconds: None,
        }
    }
}

/// Repository metadata for JSON serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RepositoryMeta {
    owner: String,
    name: String,
}

/// HackerNews metadata for JSON serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HackerNewsMeta {
    feed_type: String,
    min_score: i32,
    count: u32,
}

impl MySQLStorage {
    /// Create a new MySQL storage with default configuration
    pub async fn new() -> Result<Self, StorageError> {
        Self::with_config(&MySQLConfig::default()).await
    }

    /// Create a new MySQL storage with custom configuration
    pub async fn with_config(config: &MySQLConfig) -> Result<Self, StorageError> {
        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections.unwrap_or(DEFAULT_MAX_CONNECTIONS))
            .acquire_timeout(Duration::from_secs(
                config.timeout_seconds.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
            ))
            .connect(&config.dsn)
            .await?;

        // Ensure tables exist
        Self::initialize_database(&pool).await?;

        let storage = Self { pool };

        Ok(storage)
    }

    /// Initialize the database with required tables
    async fn initialize_database(pool: &MySqlPool) -> Result<(), StorageError> {
        // Create the sources table if it doesn't exist
        let create_source_table = r#"
            CREATE TABLE IF NOT EXISTS t_source (
                id INT PRIMARY KEY AUTO_INCREMENT,
                type VARCHAR(32) NOT NULL,
                meta JSON NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "#;

        sqlx::query(create_source_table).execute(pool).await?;

        info!("Database initialized successfully");
        Ok(())
    }

    async fn get_by_id<T, F>(
        &self,
        id: i32,
        source_type: &str,
        transform: F,
    ) -> Result<Option<T>, StorageError>
    where
        F: FnOnce(SourceRow) -> Result<T, StorageError>,
    {
        debug!("Getting {} with id: {}", source_type, id);

        let result = sqlx::query_as::<_, SourceRow>(
            "SELECT id, type, meta FROM t_source WHERE id = ? AND type = ?",
        )
        .bind(id)
        .bind(source_type)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let item = transform(row)?;
                Ok(Some(item))
            }
            None => Ok(None),
        }
    }

    async fn get_all<T, F>(&self, source_type: &str, transform: F) -> Result<Vec<T>, StorageError>
    where
        F: Fn(SourceRow) -> Result<T, StorageError>,
    {
        debug!("Getting all {}", source_type);

        let rows = sqlx::query_as::<_, SourceRow>(
            "SELECT id, type, meta FROM t_source WHERE type = ? ORDER BY id",
        )
        .bind(source_type)
        .fetch_all(&self.pool)
        .await?;

        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            match transform(row) {
                Ok(item) => items.push(item),
                Err(e) => {
                    error!("Error transforming {}: {}", source_type, e);
                    continue;
                }
            }
        }

        Ok(items)
    }

    async fn save<T, M, F>(
        &self,
        item_id: i32,
        source_type: &str,
        meta: M,
        transform: F,
    ) -> Result<T, StorageError>
    where
        M: Serialize,
        F: FnOnce(i32) -> T,
    {
        let meta_json = serde_json::to_string(&meta)?;

        let res = if item_id > 0 {
            sqlx::query(
                "
                INSERT INTO t_source (id, type, meta) VALUES (?, ?, ?)
                ON DUPLICATE KEY UPDATE meta = VALUES(meta)
                ",
            )
            .bind(item_id)
            .bind(source_type)
            .bind(&meta_json)
            .execute(&self.pool)
            .await?
        } else {
            sqlx::query(
                "
                INSERT INTO t_source (type, meta) VALUES (?, ?)
                ",
            )
            .bind(source_type)
            .bind(&meta_json)
            .execute(&self.pool)
            .await?
        };

        let id = if item_id > 0 {
            item_id
        } else {
            res.last_insert_id() as i32
        };

        Ok(transform(id))
    }

    async fn delete(&self, id: i32, source_type: &str) -> Result<(), StorageError> {
        debug!("Deleting {} with id: {}", source_type, id);

        let result = sqlx::query(
            "
            DELETE FROM t_source WHERE id = ? AND type = ?
            ",
        )
        .bind(id)
        .bind(source_type)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(SourceType::GitHub, id));
        }

        Ok(())
    }
}

#[async_trait]
impl RepositoryStorage for MySQLStorage {
    async fn get_repository(&self, id: i32) -> Result<Option<Repository>, StorageError> {
        self.get_by_id(id, "github", |row| {
            let meta: RepositoryMeta = serde_json::from_value(row.meta)?;
            Ok(Repository::new(row.id, meta.owner, meta.name))
        })
        .await
    }

    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError> {
        self.get_all("github", |row| {
            let meta: RepositoryMeta = serde_json::from_value(row.meta)?;
            Ok(Repository::new(row.id, meta.owner, meta.name))
        })
        .await
    }

    async fn get_repository_by_name(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Option<Repository>, StorageError> {
        debug!("Getting repository by name: {}/{}", owner, name);

        // 使用 JSON_EXTRACT 和 JSON_UNQUOTE 函数
        let rows = sqlx::query_as::<_, SourceRow>(
            "SELECT id, type, meta FROM t_source WHERE type = 'github' AND JSON_UNQUOTE(JSON_EXTRACT(meta, '$.owner')) = ? AND JSON_UNQUOTE(JSON_EXTRACT(meta, '$.name')) = ?"
        )
        .bind(owner)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        match rows {
            Some(row) => {
                let meta: RepositoryMeta = serde_json::from_value(row.meta)?;
                Ok(Some(Repository::new(row.id, meta.owner, meta.name)))
            }
            None => Ok(None),
        }
    }

    async fn save_repository(&self, repository: Repository) -> Result<Repository, StorageError> {
        debug!(
            "Saving repository: {}/{}",
            repository.owner, repository.name
        );

        let meta = RepositoryMeta {
            owner: repository.owner.clone(),
            name: repository.name.clone(),
        };

        self.save(repository.id, "github", meta, |id| {
            Repository::new(id, repository.owner, repository.name)
        })
        .await
    }

    async fn delete_repository(&self, id: i32) -> Result<(), StorageError> {
        self.delete(id, "github").await
    }
}

#[async_trait]
impl HackerNewsStorage for MySQLStorage {
    async fn get_hacker_news(&self, id: i32) -> Result<Option<HackerNews>, StorageError> {
        self.get_by_id(id, "hackernews", |row| {
            let meta: HackerNewsMeta = serde_json::from_value(row.meta)?;

            let feed_type = match meta.feed_type.as_str() {
                "frontpage" => HackerNewsFeedType::FrontPage,
                "newest" => HackerNewsFeedType::Newest,
                "best" => HackerNewsFeedType::Best,
                "ask" => HackerNewsFeedType::Ask,
                "show" => HackerNewsFeedType::Show,
                "jobs" => HackerNewsFeedType::Jobs,
                "polls" => HackerNewsFeedType::Polls,
                _ => HackerNewsFeedType::FrontPage, // Default to front page
            };

            Ok(HackerNews {
                id: row.id,
                feed_type,
                min_score: meta.min_score,
                count: meta.count,
            })
        })
        .await
    }

    async fn get_all_hacker_news(&self) -> Result<Vec<HackerNews>, StorageError> {
        self.get_all("hackernews", |row| {
            let meta: HackerNewsMeta = serde_json::from_value(row.meta)?;

            let feed_type = match meta.feed_type.as_str() {
                "frontpage" => HackerNewsFeedType::FrontPage,
                "newest" => HackerNewsFeedType::Newest,
                "best" => HackerNewsFeedType::Best,
                "ask" => HackerNewsFeedType::Ask,
                "show" => HackerNewsFeedType::Show,
                "jobs" => HackerNewsFeedType::Jobs,
                "polls" => HackerNewsFeedType::Polls,
                _ => HackerNewsFeedType::FrontPage, // Default to front page
            };

            Ok(HackerNews {
                id: row.id,
                feed_type,
                min_score: meta.min_score,
                count: meta.count,
            })
        })
        .await
    }

    async fn save_hacker_news(&self, hacker_news: HackerNews) -> Result<HackerNews, StorageError> {
        debug!("Saving HackerNews with id: {}", hacker_news.id);

        let feed_type_str = match hacker_news.feed_type {
            HackerNewsFeedType::FrontPage => "frontpage",
            HackerNewsFeedType::Newest => "newest",
            HackerNewsFeedType::Best => "best",
            HackerNewsFeedType::Ask => "ask",
            HackerNewsFeedType::Show => "show",
            HackerNewsFeedType::Jobs => "jobs",
            HackerNewsFeedType::Polls => "polls",
        };

        let meta = HackerNewsMeta {
            feed_type: feed_type_str.to_string(),
            min_score: hacker_news.min_score,
            count: hacker_news.count,
        };

        self.save(hacker_news.id, "hackernews", meta, |id| HackerNews {
            id,
            feed_type: hacker_news.feed_type,
            min_score: hacker_news.min_score,
            count: hacker_news.count,
        })
        .await
    }

    async fn delete_hacker_news(&self, id: i32) -> Result<(), StorageError> {
        self.delete(id, "hackernews").await
    }
}

impl Storage for MySQLStorage {}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests require a running MySQL instance
    // They are ignored by default to avoid breaking CI

    #[tokio::test]
    #[ignore = "Requires MySQL database"]
    async fn test_mysql_crud() {
        // Create a test database configuration
        let config = MySQLConfig {
            dsn: "mysql://root:root@localhost:33061/xiaotian".to_string(),
            max_connections: Some(5),
            timeout_seconds: Some(10),
        };

        // Create the storage
        let storage = MySQLStorage::with_config(&config).await.unwrap();

        // Clean up before test
        sqlx::query("DELETE FROM t_source WHERE type = 'github'")
            .execute(&storage.pool)
            .await
            .unwrap();

        // Test create
        let repo = Repository::new(0, "test-owner".to_string(), "test-repo".to_string());
        let saved_repo = storage.save_repository(repo).await.unwrap();
        assert!(saved_repo.id > 0);

        // Test read
        let fetched_repo = storage
            .get_repository(saved_repo.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(fetched_repo.owner, "test-owner");
        assert_eq!(fetched_repo.name, "test-repo");

        // Test read by name
        let fetched_by_name = storage
            .get_repository_by_name("test-owner", "test-repo")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(fetched_by_name.id, saved_repo.id);

        // Test list all
        let all_repos = storage.get_all_repositories().await.unwrap();
        assert_eq!(all_repos.len(), 1);

        // Test delete
        storage.delete_repository(saved_repo.id).await.unwrap();
        let should_be_none = storage.get_repository(saved_repo.id).await.unwrap();
        assert!(should_be_none.is_none());
    }

    #[tokio::test]
    #[ignore = "Requires MySQL database"]
    async fn test_mysql_hacker_news_crud() {
        // Create a test database configuration
        let config = MySQLConfig {
            dsn: "mysql://root:root@localhost:33061/xiaotian".to_string(),
            max_connections: Some(5),
            timeout_seconds: Some(10),
        };

        // Create the storage
        let storage = MySQLStorage::with_config(&config).await.unwrap();

        // Clean up before test
        sqlx::query("DELETE FROM t_source WHERE type = 'hackernews'")
            .execute(&storage.pool)
            .await
            .unwrap();

        // Test create
        let hn = HackerNews {
            id: 0,
            feed_type: HackerNewsFeedType::FrontPage,
            min_score: 10,
            count: 20,
        };

        let saved_hn = storage.save_hacker_news(hn).await.unwrap();
        assert!(saved_hn.id > 0);

        // Test read
        let fetched_hn = storage.get_hacker_news(saved_hn.id).await.unwrap().unwrap();
        assert_eq!(
            fetched_hn,
            HackerNews {
                id: saved_hn.id,
                feed_type: HackerNewsFeedType::FrontPage,
                min_score: 10,
                count: 20,
            }
        );

        // Test list all
        let all_hns = storage.get_all_hacker_news().await.unwrap();
        assert_eq!(all_hns.len(), 1);

        // Test update
        let mut updated_hn = fetched_hn;
        updated_hn.min_score = 15;
        let saved_updated_hn = storage.save_hacker_news(updated_hn).await.unwrap();
        assert_eq!(saved_updated_hn.min_score, 15);

        // Verify update
        let refetched_hn = storage.get_hacker_news(saved_hn.id).await.unwrap().unwrap();
        assert_eq!(
            refetched_hn,
            HackerNews {
                id: saved_hn.id,
                feed_type: HackerNewsFeedType::FrontPage,
                min_score: 15,
                count: 20,
            }
        );

        // Test delete
        storage.delete_hacker_news(saved_hn.id).await.unwrap();
        let should_be_none = storage.get_hacker_news(saved_hn.id).await.unwrap();
        assert!(should_be_none.is_none());
    }
}
