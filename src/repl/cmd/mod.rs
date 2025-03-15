mod add;
mod delete;
mod fetch;
mod list;

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use add::{AddCommands, AddRepoOpts};
pub use delete::{DeleteCommands, DeleteRepoOpts};
pub use fetch::{FetchCommands, FetchUpdatesOpts};
pub use list::{ListCommands, ListReposOpts};

pub use add::add;
pub use delete::delete;
pub use fetch::fetch;
pub use list::list;

/// Commands available in XiaoTian
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum ReplCommand {
    /// Add a repository or subscription
    #[command(subcommand)]
    Add(AddCommands),

    /// List repositories or subscriptions
    #[command(subcommand)]
    List(ListCommands),

    /// Delete a repository or subscription
    #[command(subcommand)]
    Delete(DeleteCommands),

    /// Fetch updates for subscriptions
    #[command(subcommand)]
    Fetch(FetchCommands),
}
