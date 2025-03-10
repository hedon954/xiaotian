mod cmd;
mod context;

use enum_dispatch::enum_dispatch;
use reedline_repl_rs::{CallBackMap, Repl, crossterm::style::Stylize};
use std::path::PathBuf;

use crate::{process::Processor, storage::Storage};
pub use cmd::*;
pub use context::ReplContext;

type ReplResult = Result<Option<String>, reedline_repl_rs::Error>;
pub type ReplCallbacks = reedline_repl_rs::CallBackMap<ReplContext, reedline_repl_rs::Error>;

const HISTORY_FILE: &str = ".xiaotian_history";
const HISTORY_SIZE: usize = 1000;

#[enum_dispatch]
trait CmdExector {
    async fn execute<T: Storage>(self, processor: &mut Processor<T>) -> anyhow::Result<String>;
}

/// Get command callbacks for the REPL
fn get_callbacks() -> ReplCallbacks {
    let mut callbacks = CallBackMap::new();
    callbacks.insert("add".to_string(), cmd::add);
    callbacks.insert("list".to_string(), cmd::list);
    callbacks.insert("show".to_string(), cmd::show);
    callbacks.insert("delete".to_string(), cmd::delete);
    callbacks.insert("fetch".to_string(), cmd::fetch);
    callbacks
}

const BANNER: &str = "
                 .--~~,__
    :-....,-------`~~'._.'
    `-,,,  ,_      ;'~U'
     _,-' ,'`-__; '--.     (Hi, I'm XiaoTian!)
    (_/'~~      ''''(;
";

/// Create a new REPL instance
pub fn create_repl(ctx: ReplContext) -> Repl<ReplContext, reedline_repl_rs::Error> {
    let history_file = dirs::home_dir()
        .map(|p| p.join(HISTORY_FILE))
        .unwrap_or_else(|| PathBuf::from(HISTORY_FILE));

    let callbacks = get_callbacks();

    let repl = Repl::new(ctx)
        .with_name("XiaoTian")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_prompt("xiaotian> ")
        .with_banner(BANNER.blue().to_string().as_str())
        .with_history(history_file, HISTORY_SIZE)
        .with_derived::<ReplCommand>(callbacks);
    repl
}
