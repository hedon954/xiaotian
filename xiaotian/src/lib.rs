pub mod config;
pub mod error;
pub mod llm;
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
use tracing::{info, warn};

pub async fn default_processor() -> anyhow::Result<Processor<MemoryStorage>> {
    let config = AppConfig::load("config.toml")?;
    let storage = Arc::new(MemoryStorage::new());
    let source_factory = Arc::new(DefaultSourceFactory::new(config.github.token.clone())?);
    let llm_client = init_llm_client().await?;

    // 初始化通知管理器
    let notification_manager = NotificationManager::from_config(&config);
    let notification_manager = Arc::new(notification_manager);

    let mut processor = Processor::new(storage, source_factory);
    if let Some(client) = llm_client {
        info!("Configuring LLM client: {}", client.get_name());
        processor.schedule_handler = processor
            .schedule_handler
            .with_llm_client(client)
            .with_notification_manager(notification_manager);
    } else {
        warn!("No LLM client configured, AI summaries will be disabled");
    }

    init_task(&mut processor).await?;
    Ok(processor)
}

/// 初始化 LLM 客户端
async fn init_llm_client() -> anyhow::Result<Option<Arc<dyn LLMClient>>> {
    // 尝试创建 Ollama 客户端
    let config = OllamaConfig {
        host: "http://localhost".to_string(),
        port: 11434,
        model: "llama3.2".to_string(),
        temperature: 0.3,
        top_p: 0.3,
    };

    match OllamaClient::with_config(config).await {
        Ok(client) => {
            tracing::info!("Successfully initialized Ollama LLM client");
            Ok(Some(Arc::new(client)))
        }
        Err(e) => {
            tracing::warn!("Failed to initialize Ollama LLM client: {}", e);
            tracing::warn!("AI summary generation will be disabled");
            Ok(None)
        }
    }
}

async fn init_task(processor: &mut Processor<MemoryStorage>) -> anyhow::Result<()> {
    let repos = vec![("golang".to_string(), "go".to_string())];
    for repo in repos {
        processor.add_handler.add_repository(repo.0, repo.1).await?;
    }
    Ok(())
}
