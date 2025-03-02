use std::env;
use std::sync::Arc;

use chrono::{Duration, Utc};
use octocrab::Octocrab;
use xiaotian::models::UpdateEventType;
use xiaotian::models::source::Source;
use xiaotian::sources::github::GitHubSource;

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
    let client = if let Some(token) = token {
        println!("Use GitHub token for authentication");
        Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("Failed to create GitHub client")
    } else {
        println!(
            "Warning: No GITHUB_TOKEN environment variable, using unauthenticated mode (rate limiting is strict)"
        );
        Octocrab::builder()
            .build()
            .expect("Failed to create GitHub client")
    };

    // 创建 GitHub 源
    let source = GitHubSource::new(owner.clone(), repo.clone(), None, Arc::new(client));

    println!(
        "Fetching updates for {}/{} in the last {} days...",
        owner, repo, days
    );

    // 获取更新
    let since = Utc::now() - Duration::days(days);
    match source.fetch_updates(Some(since)).await {
        Ok(updates) => {
            println!("Successfully fetched {} updates:", updates.len());

            // group by type and display
            let mut commits = vec![];
            let mut issues = vec![];
            let mut prs = vec![];
            let mut releases = vec![];
            let mut others = vec![];

            for update in &updates {
                match update.event_type {
                    UpdateEventType::Commit => commits.push(update),
                    UpdateEventType::Issue | UpdateEventType::IssueUpdate => issues.push(update),
                    UpdateEventType::PullRequest | UpdateEventType::PullRequestUpdate => {
                        prs.push(update)
                    }
                    UpdateEventType::Release => releases.push(update),
                    _ => others.push(update),
                }
            }

            println!("\nCommits ({}个):", commits.len());
            for commit in commits {
                println!(
                    "  - {}: {}",
                    commit.event_date.format("%Y-%m-%d"),
                    commit.title
                );
            }

            println!("\nIssues ({}个):", issues.len());
            for issue in issues {
                println!(
                    "  - {}: {}",
                    issue.event_date.format("%Y-%m-%d"),
                    issue.title
                );
            }

            println!("\nPull Requests ({}个):", prs.len());
            for pr in prs {
                println!("  - {}: {}", pr.event_date.format("%Y-%m-%d"), pr.title);
            }

            println!("\nReleases ({}个):", releases.len());
            for release in releases {
                println!(
                    "  - {}: {}",
                    release.event_date.format("%Y-%m-%d"),
                    release.title
                );
            }

            if !others.is_empty() {
                println!("\nOther updates ({}个):", others.len());
                for other in others {
                    println!(
                        "  - {}: {}",
                        other.event_date.format("%Y-%m-%d"),
                        other.title
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching updates: {:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
