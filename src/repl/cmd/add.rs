use clap::{ArgMatches, Parser};
use enum_dispatch::enum_dispatch;

use crate::{
    process::Processor,
    repl::{CmdExector, ReplContext, ReplResult, context::ReplMsg},
    storage::Storage,
};

/// Add commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum AddCommands {
    /// Add a repository
    Repository(AddRepoOpts),
    /// Add a subscription to a repository
    Subscription(AddSubOpts),
}

#[derive(Parser, Debug)]
pub struct AddRepoOpts {
    /// Owner of the repository
    owner: String,
    /// Name of the repository
    name: String,
}

#[derive(Parser, Debug)]
pub struct AddSubOpts {
    /// Owner of the repository
    owner: String,
    /// Name of the repository
    name: String,
}

pub fn add(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    match args.subcommand() {
        Some(("repository", args)) => add_repo(args, ctx),
        Some(("subscription", args)) => add_sub(args, ctx),
        _ => panic!("invalid subcommand"),
    }
}

fn add_repo(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = AddRepoOpts::from(args);
    let (msg, rx) = ReplMsg::new(AddCommands::Repository(opts));
    Ok(ctx.send(msg, rx))
}

fn add_sub(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = AddSubOpts::from(args);
    let (msg, rx) = ReplMsg::new(AddCommands::Subscription(opts));
    Ok(ctx.send(msg, rx))
}

impl CmdExector for AddRepoOpts {
    async fn execute<T: Storage>(self, processor: &mut Processor<T>) -> anyhow::Result<String> {
        let ret = processor
            .add_handler
            .add_repository(self.owner, self.name)
            .await?;
        Ok(ret)
    }
}

impl CmdExector for AddSubOpts {
    async fn execute<T: Storage>(self, processor: &mut Processor<T>) -> anyhow::Result<String> {
        let ret = processor
            .add_handler
            .add_subscription(self.owner, self.name)
            .await?;
        Ok(ret)
    }
}

impl From<&ArgMatches> for AddRepoOpts {
    fn from(args: &ArgMatches) -> Self {
        let owner = args.get_one::<String>("owner").unwrap();
        let name = args.get_one::<String>("name").unwrap();
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }
}

impl From<&ArgMatches> for AddSubOpts {
    fn from(args: &ArgMatches) -> Self {
        let owner = args.get_one::<String>("owner").unwrap();
        let name = args.get_one::<String>("name").unwrap();
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }
}
