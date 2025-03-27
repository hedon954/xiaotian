pub mod config;
pub mod error;
pub mod llm;
pub mod log;
pub mod models;
pub mod notification;
pub mod process;
pub mod repl;
pub mod sources;
pub mod storage;
pub mod utils;

use std::sync::Arc;

pub use config::AppConfig;
use llm::{LLMClient, OllamaClient, OllamaConfig};
pub use models::{Repository, Update};
use notification::NotificationManager;
use process::Processor;
use sources::DefaultSourceFactory;
use storage::MemoryStorage;
use tracing::info;

pub async fn default_processor(config: &AppConfig) -> anyhow::Result<Processor<MemoryStorage>> {
    let storage = Arc::new(MemoryStorage::new());
    let source_factory = Arc::new(DefaultSourceFactory::new(config.github.token.clone())?);
    let llm_client = init_llm_client().await?;

    // 初始化通知管理器
    let notification_manager = NotificationManager::from_config(config);
    let notification_manager = Arc::new(notification_manager);

    let mut processor = Processor::new(storage, source_factory);
    processor.schedule_handler = processor
        .schedule_handler
        .with_llm_client(llm_client)
        .with_notification_manager(notification_manager);

    init_task(&mut processor).await?;
    Ok(processor)
}

/// 初始化 LLM 客户端
async fn init_llm_client() -> anyhow::Result<Arc<dyn LLMClient>> {
    // 尝试创建 Ollama 客户端
    let config = OllamaConfig {
        host: "http://localhost".to_string(),
        port: 11434,
        model: "llama3.2".to_string(),
        temperature: 0.9,
        top_p: 0.9,
    };

    info!("Configuring LLM client: {}", config.model);

    let client = OllamaClient::with_config(config).await?;
    Ok(Arc::new(client))
}

async fn init_task(processor: &mut Processor<MemoryStorage>) -> anyhow::Result<()> {
    let repos = vec![("golang".to_string(), "go".to_string())];
    for repo in repos {
        processor.add_repository(repo.0, repo.1).await?;
    }
    Ok(())
}
