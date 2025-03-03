use std::sync::Arc;

use xiaotian::models::AppConfig;
use xiaotian::repl::Repl;
use xiaotian::sources::DefaultSourceFactory;
use xiaotian::storage::MemoryStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Load application configuration
    let config_path = AppConfig::get_default_path();
    let config = AppConfig::load_from_file(&config_path).unwrap_or_default();

    println!("XiaoTian v0.2.1");

    // Log GitHub token status
    if config.github_token.is_some() {
        println!("GitHub API token is configured. API rate limits will be higher.");
    } else {
        println!("No GitHub API token configured. API rate limits will apply.");
        println!("Tip: Set a token with 'config set github_token <token>'");
    }

    // Create a storage instance
    let storage = Arc::new(MemoryStorage::new());

    // Create the source factory with GitHub token from config
    let source_factory = Arc::new(
        DefaultSourceFactory::new(config.github_token.clone())
            .expect("Failed to initialize source factory"),
    );

    // Create and start the REPL
    let mut repl = Repl::new(storage, source_factory)?;
    repl.start().await?;

    Ok(())
}
