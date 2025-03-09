//! Process module for XiaoTian
//!
//! This module contains the command execution logic.

mod error;
mod handlers;

pub use error::ProcessError;
use handlers::*;

use std::sync::Arc;

use crate::command::Command;
use crate::models::AppConfig;
use crate::models::source::SourceFactory;
use crate::storage::Storage;

/// Trait for command processing
#[async_trait::async_trait]
pub trait CommandProcessor {
    /// Process a command and return the result
    async fn process(&self, command: Command) -> Result<String, ProcessError>;
}

/// Command processor implementation
pub struct CommandProcessorImpl<S: Storage> {
    storage: Arc<S>,
    source_factory: Arc<dyn SourceFactory>,
    config: AppConfig,
}

impl<S: Storage> CommandProcessorImpl<S> {
    /// Create a new command processor
    pub fn new(storage: Arc<S>, source_factory: Arc<dyn SourceFactory>) -> Self {
        let config_path = AppConfig::get_default_path();
        let config = AppConfig::load_from_file(&config_path).unwrap_or_default();

        Self {
            storage,
            source_factory,
            config,
        }
    }
}

#[async_trait::async_trait]
impl<S: Storage + Send + Sync + 'static> CommandProcessor for CommandProcessorImpl<S> {
    async fn process(&self, command: Command) -> Result<String, ProcessError> {
        match command {
            Command::Help => Ok(HelpHandler::handle()),
            Command::Add(add_command) => {
                AddHandler::new(self.storage.clone())
                    .handle(add_command)
                    .await
            }
            Command::List(list_command) => {
                ListHandler::new(self.storage.clone())
                    .handle(list_command)
                    .await
            }
            Command::Show(show_command) => {
                ShowHandler::new(self.storage.clone())
                    .handle(show_command)
                    .await
            }
            Command::Delete(delete_command) => {
                DeleteHandler::new(self.storage.clone())
                    .handle(delete_command)
                    .await
            }
            Command::Clear => ClearHandler::new(self.storage.clone()).handle().await,
            Command::Fetch(fetch_command) => {
                FetchHandler::new(
                    self.storage.clone(),
                    self.source_factory.clone(),
                    &self.config,
                )
                .handle(fetch_command)
                .await
            }
            Command::Exit => Ok("Exiting...".to_string()),
        }
    }
}
