use clap::{ArgMatches, Parser};
use enum_dispatch::enum_dispatch;

use crate::{
    process::Processor,
    repl::{CmdExector, ReplContext, ReplResult, context::ReplMsg},
};

/// Delete commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum DeleteCommands {
    /// Delete a repository
    Repo(DeleteRepoOpts),
}

#[derive(Parser, Debug)]
pub struct DeleteRepoOpts {
    /// Repository owner (username or organization)
    owner: String,

    /// Repository name
    name: String,
}

pub fn delete(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    match args.subcommand() {
        Some(("repo", args)) => delete_repo(args, ctx),
        _ => panic!("Invalid subcommand"),
    }
}

fn delete_repo(args: &ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let opts = DeleteRepoOpts::from(args);
    let (msg, rx) = ReplMsg::new(DeleteCommands::Repo(opts));
    Ok(ctx.send(msg, rx))
}

impl CmdExector for DeleteRepoOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let repo = match processor
            .list_handler
            .get_repository_by_name(&self.owner, &self.name)
            .await
        {
            Ok(repo) => repo,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Repository {}/{} not found",
                    self.owner,
                    self.name
                ));
            }
        };

        let ret = processor.delete_handler.delete_repository(repo.id).await?;
        Ok(ret)
    }
}

impl From<&ArgMatches> for DeleteRepoOpts {
    fn from(args: &ArgMatches) -> Self {
        let owner = args.get_one::<String>("owner").unwrap();
        let name = args.get_one::<String>("name").unwrap();
        Self {
            owner: owner.clone(),
            name: name.clone(),
        }
    }
}
