use xiaotian::repl::{ReplContext, create_repl};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let ctx = ReplContext::new();
    let mut repl = create_repl(ctx);
    repl.run()?;
    println!("Goodbye!");
    Ok(())
}
