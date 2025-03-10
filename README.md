# XiaoTian (哮天犬)

```
                 .--~~,__
    :-....,-------`~~'._.'
    `-,,,  ,_      ;'~U'
     _,-' ,'`-__; '--.     (XiaoTian)
    (_/'~~      ''''(;
```

> XiaoTian (哮天犬) is named after the legendary "Howling Celestial Dog" in Chinese mythology, known for its keen sense of smell and ability to track down anything. Just like its namesake, this tool is designed to "sniff out" the latest updates from GitHub repositories you care about.

XiaoTian is a command-line tool designed for developers and project managers to automatically track and summarize updates from subscribed GitHub repositories.

## Features

- Repository subscription management
- GitHub API integration with token support
- Update retrieval from GitHub repositories (commits, issues, pull requests, releases)
- Configuration management with persistent settings
- Customizable update display with filtering options
- Interactive confirmation for destructive operations
- User-friendly REPL interface with history support

## Installation

### Prerequisites

- Rust 1.70 or higher (for building from source)
- GitHub Personal Access Token (optional, for higher API rate limits)

### Installation Options

#### Option 1: Install with Cargo

The easiest way to install XiaoTian is through Cargo, Rust's package manager:

```bash
cargo install xiaotian
```

#### Option 2: Download Binary

Pre-built binaries for major platforms are available on the [Releases](https://github.com/hedon954/xiaotian/releases) page.

1. Download the appropriate binary for your platform
2. Extract the archive
3. Add the binary to your PATH

```bash
# Example for Linux/macOS
chmod +x xiaotian
sudo mv xiaotian /usr/local/bin/
```

#### Option 3: Building from Source

```bash
# Clone the repository
git clone https://github.com/hedon954/xiaotian.git
cd xiaotian

# Build the project
cargo build --release

# Run the application
cargo run --release --bin xiaotian

# Or install it locally
cargo install --path .
```

### Verifying Installation

To verify the installation, run:

```bash
xiaotian --version
```

You should see output similar to:

```
XiaoTian 0.2.6
```

## Usage

```bash
Commands available in XiaoTian

Usage: [COMMAND]

Commands:
  add     Add a repository or subscription
  list    List repositories or subscriptions
  show    Show details of a repository, subscription, or updates
  delete  Delete a repository or subscription
  fetch   Fetch updates for subscriptions
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Command Structure

XiaoTian uses a hierarchical command structure with the following main commands:

### Repository Management

- `add repository <owner> <name>` - Add a GitHub repository
- `list repositories` - List all repositories
- `show repository <id>` - Show repository details
- `delete repository <id>` - Delete a repository (with interactive confirmation)

### Subscription Management

- `add subscription <owner> <name>` - Subscribe to a repository
- `subscribe <repository_id>` - Subscribe to a repository by ID
- `list subscriptions` - List all subscriptions
- `show subscription <id>` - Show subscription details
- `delete subscription <id>` - Delete a subscription (with interactive confirmation)

### Update Operations

- `fetch updates <subscription_id> [days]` - Fetch updates for a subscription for the last N days
- `show updates <subscription_id> [limit]` - Show recent updates for a subscription with optional limit

## Development

XiaoTian is currently in version 0.2.6. Key features implemented:

- Core architecture and storage system (v0.1.0)
- GitHub API integration (v0.2.0)
- REPL commands for configuration and update management (v0.2.1)
- Command module refactoring and improved error handling (v0.2.2)
- Command line parsing with Clap integration (v0.2.3)
- Enhanced REPL experience with reedline-repl-rs (v0.2.4)
- Data relationship integrity and enhanced user experience (v0.2.5)
- ID system simplification with intuitive integer IDs (v0.2.6)

### New in v0.2.6: ID System Simplification

Version 0.2.6 focuses on improving the ID system, making it more intuitive and user-friendly:

- Replaced UUID with auto-increment integers for repositories and subscriptions
- Optimized model relationships with direct ID references
- Enhanced error handling with more specific context information
- Simplified command line interaction with easy-to-type integer IDs
- Added `subscribe <repository_id>` command for direct repository subscription

Example of the new ID system in action:

```
xiaotian> add repo rust-lang/rust
Repository added with ID: 1

xiaotian> subscribe 1
Subscription created with ID: 1

xiaotian> show subscription 1
Subscription #1:
  Repository: rust-lang/rust (ID: 1)
  Last checked: Never
  Status: Active
```

## Roadmap

XiaoTian follows a structured development roadmap:

### Upcoming Features (v0.3.0)

- Update tracking system
- Incremental update mechanism
- Scheduled update checks
- Update status notifications

### Mid-term Goals (v0.4.0 - v0.5.0)

- Notification system with multiple channels
- Report generation in various formats
- Advanced filtering capabilities
- Integration with more developer platforms

### Long-term Goals (v1.0.0+)

- Web interface with dashboard
- Team collaboration features
- Custom plugin system
- Integration with CI/CD workflows

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [Octocrab](https://github.com/XAMPPRocky/octocrab) for GitHub API integration
- [Clap](https://github.com/clap-rs/clap) for command-line argument parsing
- [reedline-repl-rs](https://github.com/kurtlawrence/reedline-repl-rs) for the interactive REPL interface
- All contributors who have helped shape and improve this project
