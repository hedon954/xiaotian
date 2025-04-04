use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, Pool, Row, mysql::MySqlPoolOptions};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;
use tracing::{debug, error, info};

use super::error::StorageError;
use super::{RepositoryStorage, Storage};
use crate::models::{Repository, SourceType};

const DEFAULT_MAX_CONNECTIONS: u32 = 5;
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

/// MySQL implementation of Storage
#[derive(Clone, Debug)]
pub struct MySQLStorage {
    /// The MySQL connection pool
    pool: Pool<MySql>,
    /// ID generator for new repositories
    source_id_gen: Arc<AtomicI32>,
}

/// Configuration options for MySQL storage
#[derive(Debug, Clone)]
pub struct MySQLConfig {
    /// Connection URL (mysql://username:password@host/database)
    pub connection_url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for MySQLConfig {
    fn default() -> Self {
        Self {
            connection_url: "mysql://xiaotian:xiaotian@localhost/xiaotian".to_string(),
            max_connections: DEFAULT_MAX_CONNECTIONS,
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
        }
    }
}

impl MySQLStorage {
    /// Create a new MySQL storage with default configuration
    pub async fn new() -> Result<Self, StorageError> {
        Self::with_config(MySQLConfig::default()).await
    }

    /// Create a new MySQL storage with custom configuration
    pub async fn with_config(config: MySQLConfig) -> Result<Self, StorageError> {
        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(config.timeout_seconds))
            .connect(&config.connection_url)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Ensure tables exist
        Self::initialize_database(&pool).await?;

        // Get the current max ID for the ID generator
        let max_id = sqlx::query("SELECT COALESCE(MAX(id), 0) as max_id FROM repositories")
            .fetch_one(&pool)
            .await
            .map_err(|e| StorageError::QueryError(e.to_string()))?
            .try_get::<i32, _>("max_id")
            .map_err(|e| StorageError::DataError(e.to_string()))?;

        let storage = Self {
            pool,
            source_id_gen: Arc::new(AtomicI32::new(max_id + 1)),
        };

        Ok(storage)
    }

    /// Initialize the database with required tables
    async fn initialize_database(pool: &MySqlPool) -> Result<(), StorageError> {
        let create_repos_table = r#"
            CREATE TABLE IF NOT EXISTS repositories (
                id INT PRIMARY KEY,
                owner VARCHAR(255) NOT NULL,
                name VARCHAR(255) NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE KEY owner_name_idx (owner, name)
            )
        "#;

        sqlx::query(create_repos_table)
            .execute(pool)
            .await
            .map_err(|e| StorageError::InitializationError(e.to_string()))?;

        info!("Database initialized successfully");
        Ok(())
    }
}

#[async_trait]
impl RepositoryStorage for MySQLStorage {
    async fn get_repository(&self, id: i32) -> Result<Option<Repository>, StorageError> {
        debug!("Getting repository with id: {}", id);

        let result = sqlx::query("SELECT id, owner, name FROM repositories WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        match result {
            Some(row) => {
                let repository = Repository::new(
                    row.try_get("id")
                        .map_err(|e| StorageError::DataError(e.to_string()))?,
                    row.try_get("owner")
                        .map_err(|e| StorageError::DataError(e.to_string()))?,
                    row.try_get("name")
                        .map_err(|e| StorageError::DataError(e.to_string()))?,
                );
                Ok(Some(repository))
            }
            None => Ok(None),
        }
    }

    async fn get_all_repositories(&self) -> Result<Vec<Repository>, StorageError> {
        debug!("Getting all repositories");

        let rows = sqlx::query("SELECT id, owner, name FROM repositories ORDER BY id")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        let mut repositories = Vec::with_capacity(rows.len());
        for row in rows {
            let repository = Repository::new(
                row.try_get("id")
                    .map_err(|e| StorageError::DataError(e.to_string()))?,
                row.try_get("owner")
                    .map_err(|e| StorageError::DataError(e.to_string()))?,
                row.try_get("name")
                    .map_err(|e| StorageError::DataError(e.to_string()))?,
            );
            repositories.push(repository);
        }

        Ok(repositories)
    }

    async fn get_repository_by_name(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Option<Repository>, StorageError> {
        debug!("Getting repository by name: {}/{}", owner, name);

        let result =
            sqlx::query("SELECT id, owner, name FROM repositories WHERE owner = ? AND name = ?")
                .bind(owner)
                .bind(name)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| StorageError::QueryError(e.to_string()))?;

        match result {
            Some(row) => {
                let repository = Repository::new(
                    row.try_get("id")
                        .map_err(|e| StorageError::DataError(e.to_string()))?,
                    row.try_get("owner")
                        .map_err(|e| StorageError::DataError(e.to_string()))?,
                    row.try_get("name")
                        .map_err(|e| StorageError::DataError(e.to_string()))?,
                );
                Ok(Some(repository))
            }
            None => Ok(None),
        }
    }

    async fn save_repository(&self, repository: Repository) -> Result<Repository, StorageError> {
        debug!(
            "Saving repository: {}/{}",
            repository.owner, repository.name
        );

        // Check if repository already exists by owner/name
        let existing = self
            .get_repository_by_name(&repository.owner, &repository.name)
            .await?;

        if let Some(existing) = existing {
            // Repository exists, update it
            sqlx::query("UPDATE repositories SET owner = ?, name = ? WHERE id = ?")
                .bind(&repository.owner)
                .bind(&repository.name)
                .bind(repository.id)
                .execute(&self.pool)
                .await
                .map_err(|e| StorageError::QueryError(e.to_string()))?;

            Ok(repository)
        } else {
            // Repository doesn't exist, insert it
            sqlx::query("INSERT INTO repositories (id, owner, name) VALUES (?, ?, ?)")
                .bind(repository.id)
                .bind(&repository.owner)
                .bind(&repository.name)
                .execute(&self.pool)
                .await
                .map_err(|e| StorageError::QueryError(e.to_string()))?;

            Ok(repository)
        }
    }

    async fn delete_repository(&self, id: i32) -> Result<(), StorageError> {
        debug!("Deleting repository with id: {}", id);

        sqlx::query("DELETE FROM repositories WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl Storage for MySQLStorage {
    async fn generate_id(&self) -> Result<i32, StorageError> {
        Ok(self.source_id_gen.fetch_add(1, Ordering::SeqCst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests require a running MySQL instance
    // They are ignored by default to avoid breaking CI

    #[tokio::test]
    #[ignore = "Requires MySQL database"]
    async fn test_mysql_crud() -> Result<(), StorageError> {
        // Create a test database configuration
        let config = MySQLConfig {
            connection_url: "mysql://xiaotian:xiaotian@localhost/xiaotian_test".to_string(),
            max_connections: 5,
            timeout_seconds: 10,
        };

        // Create the storage
        let storage = MySQLStorage::with_config(config).await?;

        // Clean up before test
        sqlx::query("DELETE FROM repositories")
            .execute(&storage.pool)
            .await
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        // Reset ID generator
        storage.source_id_gen.store(1, Ordering::SeqCst);

        // Test create
        let id = storage.generate_id().await?;
        let repo = Repository::new(id, "test-owner".to_string(), "test-repo".to_string());
        let saved_repo = storage.save_repository(repo).await?;
        assert_eq!(saved_repo.id, id);

        // Test read
        let fetched_repo = storage.get_repository(id).await?.unwrap();
        assert_eq!(fetched_repo.owner, "test-owner");
        assert_eq!(fetched_repo.name, "test-repo");

        // Test read by name
        let fetched_by_name = storage
            .get_repository_by_name("test-owner", "test-repo")
            .await?
            .unwrap();
        assert_eq!(fetched_by_name.id, id);

        // Test list all
        let all_repos = storage.get_all_repositories().await?;
        assert_eq!(all_repos.len(), 1);

        // Test delete
        storage.delete_repository(id).await?;
        let should_be_none = storage.get_repository(id).await?;
        assert!(should_be_none.is_none());

        Ok(())
    }
}
