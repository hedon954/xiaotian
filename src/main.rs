mod models;
mod repl;
mod storage;

use std::sync::Arc;

use repl::Repl;
use storage::MemoryStorage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Create a tokio runtime
    let runtime = tokio::runtime::Runtime::new()?;
    let handle = runtime.handle().clone();

    // Create a storage instance
    let storage = Arc::new(MemoryStorage::new());

    // Create and start the REPL
    let mut repl = Repl::new(storage, handle)?;
    repl.start()?;

    Ok(())
}
