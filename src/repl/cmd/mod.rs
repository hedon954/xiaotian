mod add;
mod delete;
mod fetch;
mod list;
mod show;

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use add::{AddCommands, AddRepoOpts, AddSubOpts};
pub use delete::{DeleteCommands, DeleteRepoOpts, DeleteSubOpts};
pub use fetch::{FetchCommands, FetchUpdatesOpts};
pub use list::{ListCommands, ListReposOpts, ListSubsOpts, ListUpdatesOpts};
pub use show::{ShowCommands, ShowRepoOpts, ShowSubOpts, ShowUpdatesOpts};

pub use add::add;
pub use delete::delete;
pub use fetch::fetch;
pub use list::list;
pub use show::show;

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

    /// Show details of a repository, subscription, or updates
    #[command(subcommand)]
    Show(ShowCommands),

    /// Delete a repository or subscription
    #[command(subcommand)]
    Delete(DeleteCommands),

    /// Fetch updates for subscriptions
    #[command(subcommand)]
    Fetch(FetchCommands),
}
