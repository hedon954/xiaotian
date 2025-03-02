# XiaoTian

XiaoTian is a command-line tool designed for developers and project managers to automatically track and summarize updates from subscribed GitHub repositories.

## Features

- Repository subscription management
- GitHub API integration with token support
- Update retrieval from GitHub repositories (commits, issues, pull requests, releases)
- Configuration management with persistent settings
- Customizable update display with filtering options
- User-friendly REPL interface

## Installation

### Prerequisites

- Rust 1.70 or higher
- GitHub Personal Access Token (optional, for higher API rate limits)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/hedon954/xiaotian.git
cd xiaotian

# Build the project
cargo build --release

# Run the application
cargo run --release --bin xiaotian
```

## Usage

XiaoTian provides a REPL (Read-Eval-Print Loop) interface for interacting with the tool:

```
xiaotian> help
```

### Basic Commands

- `help` - Show help message
- `add repo <owner> <name>` - Add a GitHub repository
- `add subscription <owner> <name>` - Subscribe to a repository
- `list repos` - List all repositories
- `list subscriptions` - List all subscriptions
- `show repo <owner> <name>` - Show repository details
- `show subscription <id>` - Show subscription details
- `delete repo <owner> <name>` - Delete a repository
- `delete subscription <id>` - Delete a subscription
- `clear` - Clear all data
- `exit` or `quit` - Exit the application

### New Commands in v0.2.1

- `config get <key>` - Get a configuration value (github_token, default_fetch_days, default_show_limit)
- `config set <key> <value>` - Set a configuration value
- `fetch updates <subscription_id> [days]` - Fetch updates for a subscription for the last N days
- `show updates <subscription_id> [limit]` - Show recent updates for a subscription with optional limit

## Configuration

XiaoTian supports persistent configuration through the `config` command:

```
# Set GitHub API token
xiaotian> config set github_token ghp_your_token_here

# Configure default number of days for fetch updates
xiaotian> config set default_fetch_days 14

# Configure default limit for showing updates
xiaotian> config set default_show_limit 20
```

## Examples

### Managing Subscriptions and Fetching Updates

```
# Add a subscription to the Rust repository
xiaotian> add subscription rust-lang rust

# List subscriptions
xiaotian> list subscriptions
Subscriptions:
- rust-lang/rust (8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g)

# Fetch recent updates
xiaotian> fetch updates 8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g 7

# Show updates with limit
xiaotian> show updates 8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g 10
```

## Development

XiaoTian is currently in version 0.2.1. Key features implemented:

- Core architecture and storage system (v0.1.0)
- GitHub API integration (v0.2.0)
- REPL commands for configuration and update management (v0.2.1)

Future development plans include:

- Update notification system
- Report generation
- Support for additional source types beyond GitHub

## License

This project is licensed under the MIT License - see the LICENSE file for details.
