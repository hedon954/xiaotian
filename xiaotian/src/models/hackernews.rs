use async_trait::async_trait;
use chrono::{DateTime, Utc};
use core::fmt;
use feed_rs::parser;
use reqwest::Client;
use std::str::FromStr;
use tracing::{debug, info};

use crate::error::AppError;
use crate::models::{Fetcher, SourceType, Update, UpdateEventType};

// RSS feed base URL for Hacker News
const HN_RSS_BASE_URL: &str = "https://hnrss.org";

/// HackerNews data source using RSS feeds
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HackerNews {
    /// Unique identifier for the source
    pub id: i32,
    /// Feed type (frontpage, newest, ask, etc.)
    pub feed_type: HackerNewsFeedType,
    /// Minimum score to include stories
    pub min_score: i32,
    /// Maximum number of items to fetch
    pub count: u32,
}

/// Types of Hacker News RSS feeds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HackerNewsFeedType {
    /// Front page stories
    FrontPage,
    /// Newest stories
    Newest,
    /// Best stories
    Best,
    /// Ask HN posts
    Ask,
    /// Show HN posts
    Show,
    /// Job postings
    Jobs,
    /// Polls
    Polls,
}

impl FromStr for HackerNewsFeedType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "frontpage" => Ok(HackerNewsFeedType::FrontPage),
            "newest" => Ok(HackerNewsFeedType::Newest),
            "best" => Ok(HackerNewsFeedType::Best),
            "ask" => Ok(HackerNewsFeedType::Ask),
            "show" => Ok(HackerNewsFeedType::Show),
            "jobs" => Ok(HackerNewsFeedType::Jobs),
            "polls" => Ok(HackerNewsFeedType::Polls),
            _ => Err(format!("Invalid feed type: {}", s)),
        }
    }
}

impl HackerNews {
    /// Create a new HackerNews source
    pub fn new(id: i32, feed_type: HackerNewsFeedType, min_score: i32, count: u32) -> Self {
        Self {
            id,
            feed_type,
            min_score,
            count,
        }
    }

    /// Create a new HackerNews source with default count (20 items)
    pub fn new_with_defaults(id: i32, feed_type: HackerNewsFeedType, min_score: i32) -> Self {
        Self::new(id, feed_type, min_score, 20)
    }

    /// Get the RSS feed URL with all query parameters
    fn get_feed_url(&self) -> String {
        let base_url = match self.feed_type {
            HackerNewsFeedType::FrontPage => format!("{}/frontpage.atom", HN_RSS_BASE_URL),
            HackerNewsFeedType::Newest => format!("{}/newest.atom", HN_RSS_BASE_URL),
            HackerNewsFeedType::Best => format!("{}/best.atom", HN_RSS_BASE_URL),
            HackerNewsFeedType::Ask => format!("{}/ask.atom", HN_RSS_BASE_URL),
            HackerNewsFeedType::Show => format!("{}/show.atom", HN_RSS_BASE_URL),
            HackerNewsFeedType::Jobs => format!("{}/jobs.atom", HN_RSS_BASE_URL),
            HackerNewsFeedType::Polls => format!("{}/polls.atom", HN_RSS_BASE_URL),
        };

        // Add query parameters
        format!("{}?point={}&count={}", base_url, self.min_score, self.count)
    }

    /// Get a full name for this source
    pub fn full_name(&self) -> String {
        format!("HackerNews: {}", self.feed_type)
    }
}

impl fmt::Display for HackerNewsFeedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HackerNewsFeedType::FrontPage => write!(f, "Front Page"),
            HackerNewsFeedType::Newest => write!(f, "Newest"),
            HackerNewsFeedType::Best => write!(f, "Best"),
            HackerNewsFeedType::Ask => write!(f, "Ask HN"),
            HackerNewsFeedType::Show => write!(f, "Show HN"),
            HackerNewsFeedType::Jobs => write!(f, "Jobs"),
            HackerNewsFeedType::Polls => write!(f, "Polls"),
        }
    }
}

/// Extract score from feed entry content if available
fn extract_score(content: &str) -> Option<i32> {
    // Try to find "N points" pattern in the content
    let score_pattern = " points";
    if let Some(pos) = content.find(score_pattern) {
        // Look for a number before "points"
        let score_end = pos;
        let mut score_start = pos;
        for (i, c) in content[..pos].chars().rev().enumerate() {
            if !c.is_ascii_digit() {
                score_start = pos - i;
                break;
            }
            if i == pos - 1 {
                score_start = 0;
            }
        }

        if score_start < score_end {
            if let Ok(score) = content[score_start..score_end].trim().parse::<i32>() {
                return Some(score);
            }
        }
    }
    None
}

/// Extract HN item ID from URL if available
fn extract_item_id(url: &str) -> Option<String> {
    if url.contains("news.ycombinator.com/item?id=") {
        if let Some(id_part) = url.split("id=").nth(1) {
            return Some(id_part.to_string());
        }
    }
    None
}

/// Determine update event type based on feed type and content
fn determine_event_type(feed_type: &HackerNewsFeedType, title: &str) -> UpdateEventType {
    match feed_type {
        HackerNewsFeedType::Jobs => UpdateEventType::JobPosting,
        HackerNewsFeedType::Polls => UpdateEventType::Poll,
        HackerNewsFeedType::Ask => {
            if title.starts_with("Ask HN:") {
                UpdateEventType::Question
            } else {
                UpdateEventType::Story
            }
        }
        HackerNewsFeedType::Show => {
            if title.starts_with("Show HN:") {
                UpdateEventType::Project
            } else {
                UpdateEventType::Story
            }
        }
        _ => UpdateEventType::Story,
    }
}

#[async_trait]
impl Fetcher for HackerNews {
    async fn fetch_updates(&self, since: DateTime<Utc>) -> Result<String, AppError> {
        info!(
            "Fetching HackerNews updates for feed '{}' since {}",
            self.feed_type, since
        );

        let feed_url = self.get_feed_url();
        debug!("Using RSS feed URL: {}", feed_url);

        let client = Client::new();
        let response = client
            .get(&feed_url)
            .send()
            .await
            .map_err(|e| AppError::AnyError(e.into()))?;

        let content = response
            .bytes()
            .await
            .map_err(|e| AppError::AnyError(e.into()))?;

        // Parse the RSS feed
        let feed = parser::parse(&content[..]).map_err(|e| AppError::AnyError(e.into()))?;

        debug!("Found {} entries in feed", feed.entries.len());

        // Convert feed entries to updates
        let mut updates = Vec::new();
        for entry in feed.entries {
            // Skip entries older than the since time
            if let Some(published) = entry.published {
                if published < since {
                    continue;
                }
            }

            // Get the entry link
            let link = match entry.links.first() {
                Some(link) => link.href.clone(),
                None => continue, // Skip entries without links
            };

            // Get the content and title
            let content = match entry.content {
                Some(content) => content.body.unwrap_or_default(),
                None => continue, // Skip entries without content
            };

            let title = entry
                .title
                .map_or_else(|| "Untitled".to_string(), |t| t.content.trim().to_string());

            // Extract author from the content
            let author = entry
                .authors
                .first()
                .map_or_else(|| "anonymous".to_string(), |a| a.name.clone());

            // Determine event type
            let event_type = determine_event_type(&self.feed_type, &title);

            // Extract score if available
            let score = extract_score(&content).unwrap_or(0);

            // Create additional data string
            let additional_data = format!("score: {}", score);

            // Create an update
            let update = Update {
                source_type: SourceType::HackerNews,
                event_type,
                title,
                description: content,
                url: link,
                author,
                event_date: entry.published.unwrap_or_else(Utc::now),
                fetched_at: Utc::now(),
                additional_data: Some(additional_data),
            };

            updates.push(update);
        }

        // Sort updates by date (newest first)
        updates.sort_by(|a, b| b.event_date.cmp(&a.event_date));

        // Generate markdown report
        let report = generate_report(self, &updates);

        Ok(report)
    }
}

/// Generate a markdown report for HackerNews updates
fn generate_report(source: &HackerNews, updates: &[Update]) -> String {
    let mut md = String::new();

    md.push_str(&format!("# HackerNews {} Update\n\n", source.feed_type));
    md.push_str(&format!(
        "Found {} new items with score >= {}\n\n",
        updates.len(),
        source.min_score
    ));

    if updates.is_empty() {
        md.push_str("No new items found in the specified time range.\n");
        return md;
    }

    for (i, update) in updates.iter().enumerate() {
        md.push_str(&format!("## {}. {}\n\n", i + 1, update.title));

        if let Some(ref additional_data) = update.additional_data {
            md.push_str(&format!("**{}** | ", additional_data));
        }

        md.push_str(&format!("**By: {}** | ", update.author));
        md.push_str(&format!(
            "**Posted: {}**\n\n",
            update.event_date.format("%Y-%m-%d %H:%M")
        ));

        // Add link to the story
        if update.url.contains("news.ycombinator.com") {
            md.push_str(&format!("[View on HackerNews]({})\n\n", update.url));
        } else {
            md.push_str(&format!("[Original Article]({})\n", update.url));
            // Also include HN discussion link if available
            if let Some(item_id) = extract_item_id(&update.description) {
                md.push_str(&format!(
                    "[HN Discussion](https://news.ycombinator.com/item?id={})\n\n",
                    item_id
                ));
            } else {
                md.push('\n');
            }
        }

        // Format the content based on its type
        let content_header = match update.event_type {
            UpdateEventType::Story => "### Article Content",
            UpdateEventType::Project => "### Project Description",
            UpdateEventType::Question => "### Question",
            UpdateEventType::JobPosting => "### Job Description",
            UpdateEventType::Poll => "### Poll",
            _ => "### Content",
        };

        md.push_str(&format!("{}\n\n", content_header));

        // Clean up and format the content
        let cleaned_content = clean_content(&update.description);

        // For very long content, truncate with ellipsis
        if cleaned_content.len() > 2000 {
            md.push_str(&format!(
                "{}...\n\n_(Content truncated due to length)_\n\n",
                &cleaned_content[..2000]
            ));
        } else {
            md.push_str(&format!("{}\n\n", cleaned_content));
        }

        md.push_str("---\n\n");
    }

    md
}

/// Clean up RSS content for better display
fn clean_content(content: &str) -> String {
    // Remove common HTML tags
    let content = content
        .replace("<p>", "\n\n")
        .replace("</p>", "")
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n");

    // Replace multiple newlines with just two
    let mut result = content.trim().to_string();
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_get_feed_url() {
        let hn = HackerNews::new(1, HackerNewsFeedType::FrontPage, 100, 20);
        assert_eq!(
            hn.get_feed_url(),
            "https://hnrss.org/frontpage.atom?point=100&count=20"
        );

        let hn = HackerNews::new(2, HackerNewsFeedType::Newest, 50, 30);
        assert_eq!(
            hn.get_feed_url(),
            "https://hnrss.org/newest.atom?point=50&count=30"
        );

        let hn = HackerNews::new(3, HackerNewsFeedType::Ask, 20, 15);
        assert_eq!(
            hn.get_feed_url(),
            "https://hnrss.org/ask.atom?point=20&count=15"
        );
    }

    #[test]
    fn test_extract_score() {
        assert_eq!(extract_score("This post has 123 points and..."), Some(123));
        assert_eq!(extract_score("10 points"), Some(10));
        assert_eq!(extract_score("No points here"), None);
        assert_eq!(extract_score(""), None);
    }

    #[test]
    fn test_clean_content() {
        assert_eq!(
            clean_content("<p>First paragraph</p><p>Second paragraph</p>"),
            "First paragraph\n\nSecond paragraph"
        );
        assert_eq!(
            clean_content("Line 1<br>Line 2<br/>Line 3<br />Line 4"),
            "Line 1\nLine 2\nLine 3\nLine 4"
        );
        assert_eq!(
            clean_content("Too many\n\n\n\nnewlines"),
            "Too many\n\nnewlines"
        );
    }

    #[tokio::test]
    #[ignore = "Makes external API calls"]
    async fn test_fetch_updates() {
        let hn = HackerNews::new(1, HackerNewsFeedType::FrontPage, 100, 10);
        let since = Utc::now() - Duration::days(7);

        println!("{}", hn.get_feed_url());

        let result = hn.fetch_updates(since).await;
        assert!(result.is_ok());

        let report = result.unwrap();
        println!("{}", report);
        assert!(report.contains("HackerNews Front Page Update"));
    }
}
