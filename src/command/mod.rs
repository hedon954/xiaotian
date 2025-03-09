//! Command module for XiaoTian
//!
//! This module contains the command parsing and validation logic.

mod error;

pub use error::CommandError;

use uuid::Uuid;

/// Commands supported by XiaoTian
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Show help information
    Help,

    /// Add a repository or subscription
    Add(AddCommand),

    /// List repositories or subscriptions
    List(ListCommand),

    /// Show details of a repository, subscription, or updates
    Show(ShowCommand),

    /// Delete a repository or subscription
    Delete(DeleteCommand),

    /// Clear all data
    Clear,

    /// Fetch updates for subscriptions
    Fetch(FetchCommand),

    /// Exit the application
    Exit,
}

/// Add command variants
#[derive(Debug, Clone, PartialEq)]
pub enum AddCommand {
    /// Add a repository
    Repository { owner: String, name: String },
    /// Add a subscription
    Subscription { owner: String, name: String },
}

/// List command variants
#[derive(Debug, Clone, PartialEq)]
pub enum ListCommand {
    /// List repositories
    Repositories,
    /// List subscriptions
    Subscriptions,
    /// List updates
    Updates,
}

/// Show command variants
#[derive(Debug, Clone, PartialEq)]
pub enum ShowCommand {
    /// Show repository details
    Repository { owner: String, name: String },
    /// Show subscription details
    Subscription { id: Uuid },
    /// Show updates for a subscription
    Updates {
        subscription_id: Uuid,
        limit: Option<usize>,
    },
}

/// Delete command variants
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteCommand {
    /// Delete a repository
    Repository { id: Uuid },
    /// Delete a subscription
    Subscription { id: Uuid },
}

/// Fetch command variants
#[derive(Debug, Clone, PartialEq)]
pub enum FetchCommand {
    /// Fetch updates for a subscription
    Updates {
        subscription_id: Uuid,
        days: Option<u32>,
    },
}

/// Command parser
pub struct CommandParser;

impl CommandParser {
    /// Parse a command from a string
    pub fn parse(input: &str) -> Result<Command, CommandError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CommandError::EmptyCommand);
        }

        let command = parts[0];
        let args = if parts.len() > 1 { &parts[1..] } else { &[] };

        match command {
            "help" => Ok(Command::Help),
            "add" => Self::parse_add(args),
            "list" => Self::parse_list(args),
            "show" => Self::parse_show(args),
            "delete" => Self::parse_delete(args),
            "clear" => Ok(Command::Clear),
            "fetch" => Self::parse_fetch(args),
            "exit" | "quit" => Ok(Command::Exit),
            _ => Err(CommandError::UnknownCommand(command.to_string())),
        }
    }

    /// Parse add command
    fn parse_add(args: &[&str]) -> Result<Command, CommandError> {
        if args.len() < 3 {
            return Err(CommandError::InvalidArguments(
                "add requires at least 3 arguments: <type> <owner> <name>".to_string(),
            ));
        }

        match args[0] {
            "repo" | "repository" => Ok(Command::Add(AddCommand::Repository {
                owner: args[1].to_string(),
                name: args[2].to_string(),
            })),
            "subscription" | "sub" => Ok(Command::Add(AddCommand::Subscription {
                owner: args[1].to_string(),
                name: args[2].to_string(),
            })),
            _ => Err(CommandError::InvalidArguments(format!(
                "Unknown add type: {}. Expected 'repo' or 'subscription'",
                args[0]
            ))),
        }
    }

    /// Parse list command
    fn parse_list(args: &[&str]) -> Result<Command, CommandError> {
        if args.is_empty() {
            return Err(CommandError::InvalidArguments(
                "list requires a type: repos, subscriptions, or updates".to_string(),
            ));
        }

        match args[0] {
            "repos" | "repositories" => Ok(Command::List(ListCommand::Repositories)),
            "subs" | "subscriptions" => Ok(Command::List(ListCommand::Subscriptions)),
            "updates" => Ok(Command::List(ListCommand::Updates)),
            _ => Err(CommandError::InvalidArguments(format!(
                "Unknown list type: {}. Expected 'repos', 'subscriptions', or 'updates'",
                args[0]
            ))),
        }
    }

    /// Parse show command
    fn parse_show(args: &[&str]) -> Result<Command, CommandError> {
        if args.is_empty() {
            return Err(CommandError::InvalidArguments(
                "show requires a type and arguments".to_string(),
            ));
        }

        match args[0] {
            "repo" | "repository" => {
                if args.len() < 3 {
                    return Err(CommandError::InvalidArguments(
                        "show repo requires owner and name".to_string(),
                    ));
                }
                Ok(Command::Show(ShowCommand::Repository {
                    owner: args[1].to_string(),
                    name: args[2].to_string(),
                }))
            }
            "sub" | "subscription" => {
                if args.len() < 2 {
                    return Err(CommandError::InvalidArguments(
                        "show subscription requires an ID".to_string(),
                    ));
                }
                let id = Uuid::parse_str(args[1]).map_err(|_| {
                    CommandError::InvalidArguments(format!("Invalid UUID: {}", args[1]))
                })?;
                Ok(Command::Show(ShowCommand::Subscription { id }))
            }
            "updates" => {
                if args.len() < 2 {
                    return Err(CommandError::InvalidArguments(
                        "show updates requires a subscription ID".to_string(),
                    ));
                }
                let subscription_id = Uuid::parse_str(args[1]).map_err(|_| {
                    CommandError::InvalidArguments(format!("Invalid UUID: {}", args[1]))
                })?;

                let limit = if args.len() > 2 {
                    Some(args[2].parse().map_err(|_| {
                        CommandError::InvalidArguments(format!("Invalid limit: {}", args[2]))
                    })?)
                } else {
                    None
                };

                Ok(Command::Show(ShowCommand::Updates {
                    subscription_id,
                    limit,
                }))
            }
            _ => Err(CommandError::InvalidArguments(format!(
                "Unknown show type: {}. Expected 'repo', 'subscription', or 'updates'",
                args[0]
            ))),
        }
    }

    /// Parse delete command
    fn parse_delete(args: &[&str]) -> Result<Command, CommandError> {
        if args.len() < 2 {
            return Err(CommandError::InvalidArguments(
                "delete requires a type and ID".to_string(),
            ));
        }

        match args[0] {
            "repo" | "repository" => {
                let id = Uuid::parse_str(args[1]).map_err(|_| {
                    CommandError::InvalidArguments(format!("Invalid UUID: {}", args[1]))
                })?;
                Ok(Command::Delete(DeleteCommand::Repository { id }))
            }
            "sub" | "subscription" => {
                let id = Uuid::parse_str(args[1]).map_err(|_| {
                    CommandError::InvalidArguments(format!("Invalid UUID: {}", args[1]))
                })?;
                Ok(Command::Delete(DeleteCommand::Subscription { id }))
            }
            _ => Err(CommandError::InvalidArguments(format!(
                "Unknown delete type: {}. Expected 'repo' or 'subscription'",
                args[0]
            ))),
        }
    }

    /// Parse fetch command
    fn parse_fetch(args: &[&str]) -> Result<Command, CommandError> {
        if args.len() < 2 {
            return Err(CommandError::InvalidArguments(
                "fetch requires a type and ID".to_string(),
            ));
        }

        match args[0] {
            "updates" => {
                let subscription_id = Uuid::parse_str(args[1]).map_err(|_| {
                    CommandError::InvalidArguments(format!("Invalid UUID: {}", args[1]))
                })?;

                let days = if args.len() > 2 {
                    Some(args[2].parse().map_err(|_| {
                        CommandError::InvalidArguments(format!("Invalid days: {}", args[2]))
                    })?)
                } else {
                    None
                };

                Ok(Command::Fetch(FetchCommand::Updates {
                    subscription_id,
                    days,
                }))
            }
            _ => Err(CommandError::InvalidArguments(format!(
                "Unknown fetch type: {}. Expected 'updates'",
                args[0]
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_help() {
        assert_eq!(CommandParser::parse("help").unwrap(), Command::Help);
    }

    #[test]
    fn test_parse_exit() {
        assert_eq!(CommandParser::parse("exit").unwrap(), Command::Exit);
        assert_eq!(CommandParser::parse("quit").unwrap(), Command::Exit);
    }

    #[test]
    fn test_parse_add_repository() {
        assert_eq!(
            CommandParser::parse("add repo owner name").unwrap(),
            Command::Add(AddCommand::Repository {
                owner: "owner".to_string(),
                name: "name".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_add_subscription() {
        assert_eq!(
            CommandParser::parse("add subscription owner name").unwrap(),
            Command::Add(AddCommand::Subscription {
                owner: "owner".to_string(),
                name: "name".to_string(),
            })
        );

        // Test alias
        assert_eq!(
            CommandParser::parse("add sub owner name").unwrap(),
            Command::Add(AddCommand::Subscription {
                owner: "owner".to_string(),
                name: "name".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(
            CommandParser::parse("list repos").unwrap(),
            Command::List(ListCommand::Repositories)
        );

        assert_eq!(
            CommandParser::parse("list subscriptions").unwrap(),
            Command::List(ListCommand::Subscriptions)
        );

        assert_eq!(
            CommandParser::parse("list updates").unwrap(),
            Command::List(ListCommand::Updates)
        );
    }
}
