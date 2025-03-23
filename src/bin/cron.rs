use std::sync::Arc;

use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{Layer as _, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use xiaotian::{
    AppConfig,
    llm::{OllamaClient, OllamaConfig},
    notification::NotificationManager,
    process::Processor,
    sources::DefaultSourceFactory,
    storage::MemoryStorage,
};

/// 定时任务入口
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = fmt::layer().with_filter(LevelFilter::DEBUG);
    tracing_subscriber::registry().with(layer).init();

    info!("Starting XiaoTian Scheduler v0.5.0 with LLM support");
    let local_tz = Local::from_offset(&FixedOffset::east_opt(8 * 3600).unwrap());
    let mut cron = AsyncCron::new(local_tz);

    let processor = new_processor().await?;
    cron.add_fn("0 * * * * *", move || {
        let processor = processor.schedule_handler.clone();
        async move {
            processor.run().await;
        }
    })
    .await?;

    cron.start().await;
    info!("Scheduler started successfully. Press Ctrl+C to exit.");

    tokio::signal::ctrl_c().await?;
    info!("Received Ctrl+C, shutting down...");
    Ok(())
}

async fn new_processor() -> anyhow::Result<Processor<MemoryStorage>> {
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
async fn init_llm_client() -> anyhow::Result<Option<Arc<dyn xiaotian::llm::LLMClient>>> {
    // 尝试创建 Ollama 客户端
    let config = OllamaConfig {
        host: "http://localhost".to_string(),
        port: 11434,
        model: "llama3.2".to_string(),
        temperature: 0.7,
        top_p: 0.9,
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
