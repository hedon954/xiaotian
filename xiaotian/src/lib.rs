pub mod config;
pub mod error;
pub mod llm;
pub mod log;
pub mod models;
pub mod notification;
pub mod process;
pub mod repl;
pub mod storage;
pub mod utils;

use std::sync::Arc;

pub use config::AppConfig;
use models::HackerNewsFeedType;
pub use models::{Repository, Update};
use notification::NotificationManager;
use process::Processor;
use storage::{MemoryStorage, MySQLStorage, Storage};

pub async fn memory_processor(config: &AppConfig) -> anyhow::Result<Processor<MemoryStorage>> {
    new_processor(config, MemoryStorage::new()).await
}

pub async fn mysql_processor(config: &AppConfig) -> anyhow::Result<Processor<MySQLStorage>> {
    new_processor(config, MySQLStorage::with_config(&config.mysql).await?).await
}

pub async fn new_processor<S: Storage>(config: &AppConfig, s: S) -> anyhow::Result<Processor<S>> {
    let storage = Arc::new(s);

    let notification_manager = NotificationManager::from_config(config);
    let notification_manager = Arc::new(notification_manager);

    let mut processor = Processor::new(storage);
    processor.schedule_handler = processor
        .schedule_handler
        .with_llm_client(Arc::new(config.ollama_client().await?))
        .with_llm_client(Arc::new(config.deepseek_client().await?))
        .with_notification_manager(notification_manager);

    // init_task(&mut processor).await?;
    Ok(processor)
}

#[allow(dead_code)]
async fn init_task<S: Storage>(processor: &mut Processor<S>) -> anyhow::Result<()> {
    let repos = vec![("golang".to_string(), "go".to_string())];
    for repo in repos {
        processor.add_repository(repo.0, repo.1).await?;
    }

    let hns = vec![
        HackerNewsFeedType::FrontPage,
        HackerNewsFeedType::Newest,
        HackerNewsFeedType::Ask,
        HackerNewsFeedType::Show,
        HackerNewsFeedType::Jobs,
        HackerNewsFeedType::Best,
        HackerNewsFeedType::Best,
    ];
    for hn in hns {
        processor.add_hacker_news(hn).await?;
    }
    Ok(())
}
