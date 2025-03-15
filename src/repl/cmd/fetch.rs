use clap::{ArgMatches, Parser};
use enum_dispatch::enum_dispatch;

use crate::{
    process::Processor,
    repl::{CmdExector, ReplContext, ReplResult, context::ReplMsg},
};

/// Fetch commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum FetchCommands {
    /// Fetch updates for a subscription
    Updates(FetchUpdatesOpts),
}

#[derive(Parser, Debug)]
pub struct FetchUpdatesOpts {
    /// ID of the source
    id: i32,
    /// Number of days to fetch updates for
    #[arg(short, long, default_value = "10")]
    days: u32,
}

pub fn fetch(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    match args.subcommand() {
        Some(("updates", args)) => {
            let opts = FetchUpdatesOpts::from(args);
            let (msg, rx) = ReplMsg::new(FetchCommands::Updates(opts));
            Ok(ctx.send(msg, rx))
        }
        _ => panic!("Invalid subcommand"),
    }
}

impl CmdExector for FetchUpdatesOpts {
    async fn execute<T: crate::storage::Storage>(
        self,
        processor: &mut Processor<T>,
    ) -> anyhow::Result<String> {
        let ret = processor
            .fetch_handler
            .fetch_updates(self.id, self.days)
            .await?;
        Ok(ret)
    }
}

impl From<&ArgMatches> for FetchUpdatesOpts {
    fn from(args: &ArgMatches) -> Self {
        let id = args.get_one::<i32>("id").unwrap();
        let days = args.get_one::<u32>("days").unwrap();
        Self {
            id: *id,
            days: *days,
        }
    }
}
