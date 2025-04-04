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
use storage::MemoryStorage;

pub async fn default_processor(config: &AppConfig) -> anyhow::Result<Processor<MemoryStorage>> {
    let storage = Arc::new(MemoryStorage::new());

    let notification_manager = NotificationManager::from_config(config);
    let notification_manager = Arc::new(notification_manager);

    let mut processor = Processor::new(storage);
    processor.schedule_handler = processor
        .schedule_handler
        .with_llm_client(Arc::new(config.ollama_client().await?))
        .with_llm_client(Arc::new(config.deepseek_client().await?))
        .with_notification_manager(notification_manager);

    init_task(&mut processor).await?;
    Ok(processor)
}

async fn init_task(processor: &mut Processor<MemoryStorage>) -> anyhow::Result<()> {
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
