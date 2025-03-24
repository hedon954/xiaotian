use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use xiaotian::default_processor;

/// 定时任务入口
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = fmt::layer().with_filter(LevelFilter::DEBUG);
    tracing_subscriber::registry().with(layer).init();

    info!("Starting XiaoTian Scheduler v0.5.0 with LLM support");
    let local_tz = Local::from_offset(&FixedOffset::east_opt(8 * 3600).unwrap());
    let mut cron = AsyncCron::new(local_tz);

    let processor = default_processor().await?;
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
