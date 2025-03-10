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
    /// Repository owner (username or organization)
    owner: String,

    /// Repository name
    name: String,
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
        // 使用list_handler获取仓库
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

        // 执行删除
        let ret = processor.delete_handler.delete_repository(repo.id).await?;
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
        let owner = args.get_one::<String>("owner").unwrap();
        let name = args.get_one::<String>("name").unwrap();
        Self {
            owner: owner.clone(),
            name: name.clone(),
        }
    }
}

impl From<&ArgMatches> for DeleteSubOpts {
    fn from(args: &ArgMatches) -> Self {
        let id = args.get_one::<Uuid>("id").unwrap();
        Self { id: *id }
    }
}
