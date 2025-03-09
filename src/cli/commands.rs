use clap::{Parser, Subcommand};
use uuid::Uuid;

/// XiaoTian - A GitHub Repository Tracker CLI
#[derive(Parser, Debug)]
#[command(
    name = "xiaotian",
    author = "Hedon <171725713@qq.com>",
    version,
    about = "A CLI tool for monitoring GitHub repositories.",
    long_about = "XiaoTian is a command-line tool designed for developers and project managers to automatically track and summarize updates from subscribed GitHub repositories."
)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Start in interactive REPL mode
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,
}

/// Commands available in XiaoTian
#[derive(Subcommand, Debug)]
pub enum Commands {
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

    /// Clear all data
    Clear,
}

/// Add commands
#[derive(Subcommand, Debug)]
pub enum AddCommands {
    /// Add a repository
    Repository {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        name: String,
    },
    /// Add a subscription to a repository
    Subscription {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        name: String,
    },
}

/// List commands
#[derive(Subcommand, Debug)]
pub enum ListCommands {
    /// List repositories
    Repositories,
    /// List subscriptions
    Subscriptions,
    /// List updates
    Updates,
}

/// Show commands
#[derive(Subcommand, Debug)]
pub enum ShowCommands {
    /// Show repository details
    Repository {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        name: String,
    },
    /// Show subscription details
    Subscription {
        /// ID of the subscription
        id: Uuid,
    },
    /// Show updates for a subscription
    Updates {
        /// ID of the subscription
        id: Uuid,
        /// Maximum number of updates to show
        #[arg(short, long)]
        limit: Option<usize>,
    },
}

/// Delete commands
#[derive(Subcommand, Debug)]
pub enum DeleteCommands {
    /// Delete a repository
    Repository {
        /// ID of the repository
        id: Uuid,
    },
    /// Delete a subscription
    Subscription {
        /// ID of the subscription
        id: Uuid,
    },
}

/// Fetch commands
#[derive(Subcommand, Debug)]
pub enum FetchCommands {
    /// Fetch updates for a subscription
    Updates {
        /// ID of the subscription
        id: Uuid,
        /// Number of days to fetch updates for
        #[arg(short, long)]
        days: Option<u32>,
    },
}
