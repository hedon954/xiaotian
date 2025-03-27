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
    Repos(ListReposOpts),
}

#[derive(Parser, Debug)]
pub struct ListReposOpts;

pub fn list(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    match args.subcommand() {
        Some(("repos", args)) => list_repo(args, ctx),
        _ => panic!("Invalid subcommand"),
    }
}

fn list_repo(_args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let (msg, rx) = ReplMsg::new(ListCommands::Repos(ListReposOpts));
    Ok(ctx.send(msg, rx))
}

impl CmdExector for ListReposOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let repos = processor.list_repositories().await?;
        let mut result = String::new();
        for repo in repos {
            result.push_str(&format!("{} {}\n", repo.id, repo.name));
        }
        Ok(result)
    }
}
