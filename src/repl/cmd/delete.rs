use clap::{ArgMatches, Parser};
use enum_dispatch::enum_dispatch;
use uuid::Uuid;

use crate::{
    process::Processor,
    repl::{CmdExector, ReplContext, ReplResult, context::ReplMsg},
};

/// Delete commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum DeleteCommands {
    /// Delete a repository
    Repository(DeleteRepoOpts),
    /// Delete a subscription
    Subscription(DeleteSubOpts),
}

#[derive(Parser, Debug)]
pub struct DeleteRepoOpts {
    /// ID of the repository
    id: Uuid,
}

#[derive(Parser, Debug)]
pub struct DeleteSubOpts {
    /// ID of the subscription
    id: Uuid,
}

pub fn delete(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    match args.subcommand() {
        Some(("repository", args)) => delete_repo(args, ctx),
        Some(("subscription", args)) => delete_sub(args, ctx),
        _ => panic!("Invalid subcommand"),
    }
}

fn delete_repo(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = DeleteRepoOpts::from(args);
    let (msg, rx) = ReplMsg::new(DeleteCommands::Repository(opts));
    Ok(ctx.send(msg, rx))
}

fn delete_sub(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = DeleteSubOpts::from(args);
    let (msg, rx) = ReplMsg::new(DeleteCommands::Subscription(opts));
    Ok(ctx.send(msg, rx))
}

impl CmdExector for DeleteRepoOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor.delete_handler.delete_repository(self.id).await?;
        Ok(ret)
    }
}

impl CmdExector for DeleteSubOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor
            .delete_handler
            .delete_subscription(self.id)
            .await?;
        Ok(ret)
    }
}

impl From<&ArgMatches> for DeleteRepoOpts {
    fn from(args: &ArgMatches) -> Self {
        let id = args.get_one::<Uuid>("id").unwrap();
        Self { id: *id }
    }
}

impl From<&ArgMatches> for DeleteSubOpts {
    fn from(args: &ArgMatches) -> Self {
        let id = args.get_one::<Uuid>("id").unwrap();
        Self { id: *id }
    }
}
