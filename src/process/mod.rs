//! Command handlers module

pub mod add;
pub mod delete;
pub mod fetch;
pub mod list;
pub mod schedule;

use std::sync::Arc;

use crate::{models::source::SourceFactory, storage::Storage};

use self::{
    add::AddHandler, delete::DeleteHandler, fetch::FetchHandler, list::ListHandler,
    schedule::ScheduleHandler,
};

/// Processor for handling commands
pub struct Processor<S: Storage> {
    pub add_handler: AddHandler<S>,
    pub delete_handler: DeleteHandler<S>,
    pub fetch_handler: FetchHandler<S>,
    pub list_handler: ListHandler<S>,
    pub schedule_handler: ScheduleHandler<S>,
}

impl<S: Storage> Processor<S> {
    /// Create a new processor
    pub fn new(storage: Arc<S>, source_factory: Arc<dyn SourceFactory>) -> Self {
        Self {
            add_handler: AddHandler::new(storage.clone()),
            delete_handler: DeleteHandler::new(storage.clone()),
            fetch_handler: FetchHandler::new(storage.clone(), source_factory.clone()),
            list_handler: ListHandler::new(storage.clone()),
            schedule_handler: ScheduleHandler::new(storage.clone(), source_factory, None),
        }
    }
}
