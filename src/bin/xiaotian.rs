use std::sync::Arc;

use clap::Parser;
use colored::Colorize;

use xiaotian::Cli;
use xiaotian::cli::adapter::CommandAdapter;
use xiaotian::models::AppConfig;
use xiaotian::process::{CommandProcessor, CommandProcessorImpl};
use xiaotian::repl::Repl;
use xiaotian::sources::DefaultSourceFactory;
use xiaotian::storage::MemoryStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Load application configuration
    let config_path = AppConfig::get_default_path();
    let config = AppConfig::load_from_file(&config_path).unwrap_or_default();

    // Print application header
    println!("{}", "XiaoTian v0.2.3".bright_green());

    // Log GitHub token status
    if config.github_token.is_some() {
        println!(
            "{}",
            "GitHub API token is configured. API rate limits will be higher.".bright_blue()
        );
    } else {
        println!(
            "{}",
            "No GitHub API token configured. API rate limits will apply.".yellow()
        );
        println!(
            "{}",
            "Tip: Set a token with 'config set github_token <token>'".bright_yellow()
        );
    }

    // Create a storage instance
    let storage = Arc::new(MemoryStorage::new());

    // Create the source factory with GitHub token from config
    let source_factory = Arc::new(
        DefaultSourceFactory::new(config.github_token.clone())
            .expect("Failed to initialize source factory"),
    );

    // Determine whether to run in interactive mode or process a single command
    if cli.interactive || cli.command.is_none() {
        // Start interactive REPL mode
        let mut repl = Repl::new(storage, source_factory)?;
        repl.start().await?;
    } else if let Some(internal_command) = CommandAdapter::adapt(cli.command) {
        // Process a single command in CLI mode
        let processor = CommandProcessorImpl::new(storage, source_factory);
        match processor.process(internal_command).await {
            Ok(result) => println!("{}", result),
            Err(err) => eprintln!("{}: {}", "Error".bright_red(), err),
        }
    }

    Ok(())
}
