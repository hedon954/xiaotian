use std::env;

use chrono::{Duration, Local};
use xiaotian::{
    Repository,
    models::{Fetcher, Source},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <owner> <repo> [days]", args[0]);
        std::process::exit(1);
    }

    let owner = args[1].clone();
    let repo = args[2].clone();
    let days = if args.len() > 3 {
        args[3].parse::<i64>().unwrap_or(7)
    } else {
        7
    };

    // create GitHub client
    let token = env::var("GITHUB_TOKEN").ok();

    // create GitHub source
    let mut repo = Repository::new(1, owner.clone(), repo.clone());
    if let Some(token) = token {
        repo = repo.with_token(token);
    }
    let source = Source::GitHub(repo);

    println!(
        "Fetching updates for {} in the last {} days...",
        source.get_name(),
        days
    );

    // get updates
    let since = Local::now() - Duration::days(days);
    match source.fetch_updates(since).await {
        Ok(updates) => {
            println!(
                "Successfully fetched {} updates:\n{}",
                source.get_name(),
                updates
            );
        }
        Err(e) => {
            eprintln!("Error fetching updates: {:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
