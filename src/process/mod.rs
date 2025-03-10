//! Command handlers module

mod add;
mod delete;
mod fetch;
mod list;
mod show;

use std::sync::Arc;

pub use add::AddHandler;
pub use delete::DeleteHandler;
pub use fetch::FetchHandler;
pub use list::ListHandler;
pub use show::ShowHandler;

use crate::{models::SourceFactory, storage::Storage};

#[derive(Clone)]
pub struct Processor<S: Storage> {
    pub fetch_handler: FetchHandler<S>,
    pub list_handler: ListHandler<S>,
    pub show_handler: ShowHandler<S>,
    pub add_handler: AddHandler<S>,
    pub delete_handler: DeleteHandler<S>,
}

impl<S: Storage> Processor<S> {
    pub fn new(storage: Arc<S>, source_factory: Arc<dyn SourceFactory>) -> Self {
        Self {
            fetch_handler: FetchHandler::new(storage.clone(), source_factory),
            list_handler: ListHandler::new(storage.clone()),
            show_handler: ShowHandler::new(storage.clone()),
            add_handler: AddHandler::new(storage.clone()),
            delete_handler: DeleteHandler::new(storage.clone()),
        }
    }
}
