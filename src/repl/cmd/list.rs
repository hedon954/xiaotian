use clap::{ArgMatches, Parser};
use enum_dispatch::enum_dispatch;

use crate::{
    process::Processor,
    repl::{CmdExector, ReplContext, ReplResult, context::ReplMsg},
};

/// List commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum ListCommands {
    /// List repositories
    Repositories(ListReposOpts),
    /// List subscriptions
    Subscriptions(ListSubsOpts),
    /// List updates
    Updates(ListUpdatesOpts),
}

#[derive(Parser, Debug)]
pub struct ListReposOpts;

#[derive(Parser, Debug)]
pub struct ListSubsOpts;

#[derive(Parser, Debug)]
pub struct ListUpdatesOpts;

pub fn list(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    match args.subcommand() {
        Some(("repositories", args)) => list_repo(args, ctx),
        Some(("subscriptions", args)) => list_sub(args, ctx),
        Some(("updates", args)) => list_updates(args, ctx),
        _ => panic!("Invalid subcommand"),
    }
}

fn list_repo(_args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let (msg, rx) = ReplMsg::new(ListCommands::Repositories(ListReposOpts));
    Ok(ctx.send(msg, rx))
}

fn list_sub(_args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let (msg, rx) = ReplMsg::new(ListCommands::Subscriptions(ListSubsOpts));
    Ok(ctx.send(msg, rx))
}

fn list_updates(_args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let (msg, rx) = ReplMsg::new(ListCommands::Updates(ListUpdatesOpts));
    Ok(ctx.send(msg, rx))
}

impl CmdExector for ListReposOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor.list_handler.list_repositories().await?;
        Ok(ret)
    }
}

impl CmdExector for ListSubsOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor.list_handler.list_subscriptions().await?;
        Ok(ret)
    }
}

impl CmdExector for ListUpdatesOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor.list_handler.list_updates().await?;
        Ok(ret)
    }
}
