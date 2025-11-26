use chrono::{DateTime, Utc, NaiveDateTime, Datelike};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use url::Url;
use validator::Validate;

use crate::shared::error::AppError;

pub fn validate_feed_url(url: &str) -> Result<(), AppError> {
    if url.trim().is_empty() {
        return Err(AppError::FeedUrlInvalid("URL cannot be empty".to_string()));
    }

    let parsed_url = Url::parse(url)
        .map_err(|_| AppError::FeedUrlInvalid("Invalid URL format".to_string()))?;

    if !matches!(parsed_url.scheme(), "http" | "https") {
        return Err(AppError::FeedUrlInvalid("URL must use HTTP or HTTPS scheme".to_string()));
    }

    if parsed_url.host().is_none() {
        return Err(AppError::FeedUrlInvalid("URL must have a valid host".to_string()));
    }

    Ok(())
}

pub fn validate_cron_expression(expression: &str) -> Result<(), AppError> {
    if expression.trim().is_empty() {
        return Err(AppError::CronExpressionInvalid("Cron expression cannot be empty".to_string()));
    }

    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() != 5 {
        return Err(AppError::CronExpressionInvalid("Cron expression must have exactly 5 fields".to_string()));
    }

    // Basic validation for each field
    let fields = ["minute", "hour", "day", "month", "day_of_week"];
    for (i, (field, value)) in fields.iter().zip(parts.iter()).enumerate() {
        if !is_valid_cron_field(value, i) {
            return Err(AppError::CronExpressionInvalid(
                format!("Invalid {} field: {}", field, value)
            ));
        }
    }

    Ok(())
}

fn is_valid_cron_field(value: &str, field_index: usize) -> bool {
    let ranges = [
        (0, 59),   // minute
        (0, 23),   // hour
        (1, 31),   // day
        (1, 12),   // month
        (0, 7),    // day of week (0 and 7 both represent Sunday)
    ];

    let (min, max) = ranges[field_index];

    // Allow * (any value)
    if value == "*" {
        return true;
    }

    // Allow numbers
    if let Ok(num) = value.parse::<u32>() {
        return num >= min && num <= max;
    }

    // Allow step values (e.g., */5, 1-10/2)
    if value.contains('/') {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() != 2 {
            return false;
        }

        let base = parts[0];
        let step = parts[1];

        if let Ok(step_num) = step.parse::<u32>() {
            if step_num == 0 {
                return false;
            }

            if base == "*" {
                return true;
            }

            // Validate range (e.g., 1-10)
            if base.contains('-') {
                let range_parts: Vec<&str> = base.split('-').collect();
                if range_parts.len() != 2 {
                    return false;
                }

                if let (Ok(start), Ok(end)) = (
                    range_parts[0].parse::<u32>(),
                    range_parts[1].parse::<u32>()
                ) {
                    return start >= min && end <= max && start <= end;
                }
            }

            return false;
        }
    }

    // Allow ranges (e.g., 1-5)
    if value.contains('-') {
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 2 {
            return false;
        }

        if let (Ok(start), Ok(end)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            return start >= min && end <= max && start <= end;
        }
    }

    // Allow comma-separated values
    if value.contains(',') {
        let values: Vec<&str> = value.split(',').collect();
        for val in values {
            if !is_valid_cron_field(val.trim(), field_index) {
                return false;
            }
        }
        return true;
    }

    false
}

pub fn sanitize_html(html: &str) -> String {
    // Basic HTML sanitization - remove tags but keep content
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

pub fn extract_text_from_html(html: &str, max_length: Option<usize>) -> String {
    let text = sanitize_html(html);
    let cleaned = text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    match max_length {
        Some(length) => {
            if cleaned.len() <= length {
                cleaned
            } else {
                format!("{}...", &cleaned[..length])
            }
        }
        None => cleaned
    }
}

pub fn format_chinese_date(date: &DateTime<Utc>) -> String {
    let date_naive = date.naive_utc();
    format!("{}年{}月{}日",
        date_naive.year(),
        date_naive.month(),
        date_naive.day()
    )
}

pub fn generate_unique_id(prefix: &str) -> String {
    format!("{}-{}", prefix, uuid::Uuid::new_v4())
}

pub fn parse_tags(tags_str: &str) -> Vec<String> {
    if tags_str.trim().is_empty() {
        return Vec::new();
    }

    let mut tags: HashSet<String> = HashSet::new();

    for tag in tags_str.split(',') {
        let cleaned = tag.trim().to_string();
        if !cleaned.is_empty() && cleaned.len() <= 50 {
            tags.insert(cleaned);
        }
    }

    tags.into_iter().collect()
}

pub fn format_tags(tags: &[String]) -> String {
    tags.join(", ")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailValidation {
    pub email: String,
}

impl Validate for EmailValidation {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if !validator::validate_email(&self.email) {
            errors.add(
                "email",
                validator::ValidationError::new("invalid_email")
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub fn validate_email_addresses(emails: &[String]) -> Result<(), AppError> {
    if emails.is_empty() {
        return Err(AppError::EmailConfigInvalid);
    }

    for email in emails {
        let validation = EmailValidation { email: email.clone() };
        if let Err(_) = validation.validate() {
            return Err(AppError::EmailConfigInvalid);
        }
    }

    Ok(())
}

pub fn parse_duration_from_string(duration_str: &str) -> Result<std::time::Duration, AppError> {
    let re = Regex::new(r"^(?P<number>\d+)(?P<unit>ms|s|m|h|d)$")
        .map_err(|_| AppError::CronExpressionInvalid("Invalid duration format".to_string()))?;

    if let Some(captures) = re.captures(duration_str) {
        let number: u64 = captures["number"].parse()
            .map_err(|_| AppError::CronExpressionInvalid("Invalid number in duration".to_string()))?;

        let unit = &captures["unit"];

        let duration = match unit {
            "ms" => std::time::Duration::from_millis(number),
            "s" => std::time::Duration::from_secs(number),
            "m" => std::time::Duration::from_secs(number * 60),
            "h" => std::time::Duration::from_secs(number * 3600),
            "d" => std::time::Duration::from_secs(number * 86400),
            _ => return Err(AppError::CronExpressionInvalid("Invalid duration unit".to_string())),
        };

        Ok(duration)
    } else {
        Err(AppError::CronExpressionInvalid("Duration must be in format like '30s', '5m', '1h'".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_feed_url() {
        assert!(validate_feed_url("https://example.com/feed.xml").is_ok());
        assert!(validate_feed_url("http://example.com/rss").is_ok());
        assert!(validate_feed_url("ftp://example.com/feed.xml").is_err());
        assert!(validate_feed_url("not-a-url").is_err());
        assert!(validate_feed_url("").is_err());
    }

    #[test]
    fn test_validate_cron_expression() {
        assert!(validate_cron_expression("0 9 * * *").is_ok()); // Daily at 9 AM
        assert!(validate_cron_expression("*/5 * * * *").is_ok()); // Every 5 minutes
        assert!(validate_cron_expression("0 9 * * 1-5").is_ok()); // Weekdays at 9 AM
        assert!(validate_cron_expression("").is_err());
        assert!(validate_cron_expression("0").is_err());
        assert!(validate_cron_expression("0 25 * * *").is_err()); // Invalid hour
    }

    #[test]
    fn test_parse_tags() {
        let tags = parse_tags("rust, programming, AI, 机器学习");
        assert_eq!(tags.len(), 4);
        assert!(tags.contains(&"rust".to_string()));
        assert!(tags.contains(&"programming".to_string()));

        let empty_tags = parse_tags("");
        assert!(empty_tags.is_empty());

        let duplicate_tags = parse_tags("rust, rust, programming");
        assert_eq!(duplicate_tags.len(), 2);
    }

    #[test]
    fn test_format_chinese_date() {
        let date = "2025-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let formatted = format_chinese_date(&date);
        assert_eq!(formatted, "2025年1月15日");
    }
}