# XiaoTian

XiaoTian is a command-line tool designed for developers and project managers to automatically track and summarize updates from subscribed GitHub repositories.

## Features

- Repository subscription management
- Update retrieval from GitHub repositories
- Notification system for new updates
- Report generation for repository activities

## Installation

### Prerequisites

- Rust 1.70 or higher

### Building from Source

```bash
# Clone the repository
git clone https://github.com/hedon954/xiaotian.git
cd xiaotian

# Build the project
cargo build --release

# Run the application
cargo run --release
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

## Development

XiaoTian is currently in version 0.1.0, focusing on the core architecture and basic functionality.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
