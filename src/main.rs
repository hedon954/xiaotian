mod models;
mod repl;
mod sources;
mod storage;

use std::sync::Arc;

use repl::Repl;
use sources::DefaultSourceFactory;
use storage::MemoryStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Create a tokio runtime
    let runtime = tokio::runtime::Runtime::new()?;
    let handle = runtime.handle().clone();

    // Create a storage instance
    let storage = Arc::new(MemoryStorage::new());

    // Create the source factory (without authentication for now)
    let source_factory =
        Arc::new(DefaultSourceFactory::new(None).expect("Failed to initialize source factory"));

    // Create and start the REPL
    let mut repl = Repl::new(storage, source_factory, handle)?;
    repl.start().await?;

    Ok(())
}
