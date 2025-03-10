use clap::{ArgMatches, Parser};
use enum_dispatch::enum_dispatch;
use uuid::Uuid;

use crate::{
    process::Processor,
    repl::{CmdExector, ReplContext, ReplResult, context::ReplMsg},
};

/// Show commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum ShowCommands {
    /// Show a repository
    Repository(ShowRepoOpts),
    /// Show a subscription
    Subscription(ShowSubOpts),
    /// Show updates for a subscription
    Updates(ShowUpdatesOpts),
}

#[derive(Parser, Debug)]
pub struct ShowRepoOpts {
    /// ID of the repository
    owner: String,
    /// Name of the repository
    name: String,
}

#[derive(Parser, Debug)]
pub struct ShowSubOpts {
    /// ID of the subscription
    id: Uuid,
}

#[derive(Parser, Debug)]
pub struct ShowUpdatesOpts {
    /// ID of the subscription
    sub_id: Uuid,
    /// Limit the number of updates to show
    limit: usize,
}

pub fn show(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    match args.subcommand() {
        Some(("repositories", args)) => show_repo(args, ctx),
        Some(("subscriptions", args)) => show_sub(args, ctx),
        Some(("updates", args)) => show_updates(args, ctx),
        _ => panic!("Invalid subcommand"),
    }
}

fn show_repo(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = ShowRepoOpts::from(args);
    let (msg, rx) = ReplMsg::new(ShowCommands::Repository(opts));
    Ok(ctx.send(msg, rx))
}

fn show_sub(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = ShowSubOpts::from(args);
    let (msg, rx) = ReplMsg::new(ShowCommands::Subscription(opts));
    Ok(ctx.send(msg, rx))
}

fn show_updates(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = ShowUpdatesOpts::from(args);
    let (msg, rx) = ReplMsg::new(ShowCommands::Updates(opts));
    Ok(ctx.send(msg, rx))
}

impl CmdExector for ShowRepoOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor
            .show_handler
            .show_repository(self.owner, self.name)
            .await?;
        Ok(ret)
    }
}

impl CmdExector for ShowSubOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor.show_handler.show_subscription(self.id).await?;
        Ok(ret)
    }
}

impl CmdExector for ShowUpdatesOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor
            .show_handler
            .show_updates(self.sub_id, self.limit)
            .await?;
        Ok(ret)
    }
}

impl From<&ArgMatches> for ShowRepoOpts {
    fn from(args: &ArgMatches) -> Self {
        let owner = args.get_one::<String>("owner").unwrap();
        let name = args.get_one::<String>("name").unwrap();
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }
}

impl From<&ArgMatches> for ShowSubOpts {
    fn from(args: &ArgMatches) -> Self {
        let id = args.get_one::<Uuid>("id").unwrap();
        Self { id: *id }
    }
}

impl From<&ArgMatches> for ShowUpdatesOpts {
    fn from(args: &ArgMatches) -> Self {
        let sub_id = args.get_one::<Uuid>("sub_id").unwrap();
        let limit = args.get_one::<usize>("limit").unwrap_or(&10);
        Self {
            sub_id: *sub_id,
            limit: *limit,
        }
    }
}
