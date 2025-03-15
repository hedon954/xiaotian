use std::sync::Arc;

use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use xiaotian::{process::Processor, sources::DefaultSourceFactory, storage::MemoryStorage};

/// 定时任务入口
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("Starting XiaoTian Scheduler v0.3.0");
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
    println!("Scheduler started successfully. Press Ctrl+C to exit.");

    tokio::signal::ctrl_c().await?;
    println!("Received Ctrl+C, shutting down...");
    Ok(())
}

async fn new_processor() -> anyhow::Result<Processor<MemoryStorage>> {
    let storage = Arc::new(MemoryStorage::new());
    let source_factory = Arc::new(DefaultSourceFactory::new(None)?);
    let mut processor = Processor::new(storage, source_factory);
    init_task(&mut processor).await?;
    Ok(processor)
}

async fn init_task(processor: &mut Processor<MemoryStorage>) -> anyhow::Result<()> {
    let repos = vec![
        ("hedon954".to_string(), "goapm".to_string()),
        ("hedon954".to_string(), "xiaotian".to_string()),
        ("rust-lang".to_string(), "rust".to_string()),
    ];
    for repo in repos {
        processor.add_handler.add_repository(repo.0, repo.1).await?;
    }
    Ok(())
}
