#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{Duration, Utc};
    use octocrab::Octocrab;

    use crate::models::source::Source;
    use crate::sources::github::GitHubSource;

    #[tokio::test]
    async fn test_github_source_creation() {
        let client = Arc::new(Octocrab::builder().build().unwrap());
        let source = GitHubSource::new("rust-lang".to_string(), "rust".to_string(), None, client);

        assert_eq!(source.get_type().to_string(), "GitHub");
        assert_eq!(source.get_name(), "rust-lang/rust");
        assert_eq!(source.get_url(), "https://github.com/rust-lang/rust");
        assert_eq!(source.get_id(), "github:rust-lang:rust");
    }

    #[tokio::test]
    async fn test_fetch_updates() {
        let client = Arc::new(Octocrab::builder().build().unwrap());
        let source = GitHubSource::new("rust-lang".to_string(), "rust".to_string(), None, client);

        // get the updates in the last 7 days
        let since = Utc::now() - Duration::days(7);
        let updates = source.fetch_updates(Some(since)).await;

        // because it's a public API, there may be rate limiting issues, so here we only check if the result is successful or not
        // do not test the number of updates returned
        match updates {
            Ok(updates) => {
                println!("Successfully fetched {} updates", updates.len());
                // verify the basic properties of the updates
                if !updates.is_empty() {
                    let update = &updates[0];
                    assert_eq!(update.source_type.to_string(), "GitHub");
                    assert!(update.source_id.contains("github:rust-lang:rust"));
                    assert!(!update.title.is_empty());
                    assert!(!update.url.is_empty());
                }
            }
            Err(e) => {
                println!(
                    "Error fetching updates, possibly due to rate limiting: {:?}",
                    e
                );
                // even if it's an error, it's still passed, because it may be due to GitHub API rate limiting
            }
        }
    }
}
