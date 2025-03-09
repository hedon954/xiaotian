//! Clear command handler

use std::sync::Arc;

use crate::process::ProcessError;
use crate::storage::Storage;

/// Handler for clear command
pub struct ClearHandler<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> ClearHandler<S> {
    /// Create a new clear handler
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Handle the clear command
    pub async fn handle(&self) -> Result<String, ProcessError> {
        self.storage.clear().await?;
        Ok("All data cleared.".to_string())
    }
}
