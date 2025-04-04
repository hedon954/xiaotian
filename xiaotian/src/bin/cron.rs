use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use tracing::{error, info};
use xiaotian::{AppConfig, log::init_logger, mysql_processor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    info!("Starting XiaoTian Scheduler v0.8.0 with LLM support");
    let local_tz = Local::from_offset(&FixedOffset::east_opt(8 * 3600).unwrap());
    let mut cron = AsyncCron::new(local_tz);

    let config = AppConfig::load("config.toml")?;
    let processor = mysql_processor(&config).await?;
    cron.add_fn("0 * * * * *", move || {
        let processor = processor.clone();
        let config = config.clone();
        async move {
            let _ = processor
                .run_all(
                    "deepseek-chat".to_string(),
                    config.notification.email.unwrap().to,
                )
                .await
                .map_err(|e| error!("Error running processor: {}", e));
        }
    })
    .await?;

    cron.start().await;
    info!("Scheduler started successfully. Press Ctrl+C to exit.");

    tokio::signal::ctrl_c().await?;
    info!("Received Ctrl+C, shutting down...");
    Ok(())
}
