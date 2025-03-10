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
XiaoTian 0.2.5
```

## Usage

XiaoTian provides both a REPL (Read-Eval-Print Loop) interface and a traditional CLI mode:

### REPL Mode

Start the REPL interface:

```bash
xiaotian
```

Once in the REPL, use the `help` command to see available commands:

```
xiaotian> help
```

### CLI Mode

Run single commands directly:

```bash
xiaotian add repository rust-lang rust
```

## Command Structure

XiaoTian uses a hierarchical command structure with the following main commands:

### Repository Management

- `add repository <owner> <name>` - Add a GitHub repository
- `list repositories` - List all repositories
- `show repository <owner> <name>` - Show repository details
- `delete repository <owner> <name>` - Delete a repository (with interactive confirmation)

### Subscription Management

- `add subscription <owner> <name>` - Subscribe to a repository
- `list subscriptions` - List all subscriptions
- `show subscription <id>` - Show subscription details
- `delete subscription <id>` - Delete a subscription (with interactive confirmation)

### Update Operations

- `fetch updates <subscription_id> [days]` - Fetch updates for a subscription for the last N days
- `show updates <subscription_id> [limit]` - Show recent updates for a subscription with optional limit

### Configuration

- `config get <key>` - Get a configuration value (github_token, default_fetch_days, default_show_limit)
- `config set <key> <value>` - Set a configuration value

### System Commands

- `help` - Show help message with available commands
- `exit` or `quit` - Exit the REPL
- `clear` - Clear the terminal screen

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

### Managing Repositories and Subscriptions

```
# Add a GitHub repository
xiaotian> add repository rust-lang rust
Repository added: rust-lang/rust

# Add a subscription to the repository
xiaotian> add subscription rust-lang rust
Subscription added for: rust-lang/rust (8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g)

# List all repositories
xiaotian> list repositories
Repositories:
- rust-lang/rust (Last updated: 2025-04-10)

# List all subscriptions
xiaotian> list subscriptions
Subscriptions:
- rust-lang/rust (8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g)
```

### Fetching and Viewing Updates

```
# Fetch recent updates (last 7 days)
xiaotian> fetch updates 8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g 7
Found 45 updates:

Commits (20):
  - [2025-04-01] Update dependencies
  - [2025-03-30] Fix memory leak in compiler
  ...

Issues (15):
  - [2025-04-02] ICE when using async traits with dyn
  ...

Pull Requests (8):
  - [2025-04-02] #98765: Implement RFC 3456
  - [2025-04-01] #98764: Fix regression in pattern matching
  ...

Releases (2):
  - [2025-04-01] Rust 1.75.0
  - [2025-03-28] Rust 1.74.1
  ...

# Show updates with limit
xiaotian> show updates 8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g 10
Showing 10 most recent updates for rust-lang/rust:

1. [COMMIT] 2025-04-01: Update dependencies (a1b2c3d)
2. [ISSUE] 2025-04-02: ICE when using async traits with dyn (#12345)
3. [PR] 2025-04-02: Implement RFC 3456 (#98765)
...
```

### Interactive Deletion Workflow

```
xiaotian> delete repository rust-lang rust
This repository has 2 related subscriptions:
- rust-lang/rust (8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g)
- rust-lang/rust [Weekly] (9e2f8c53-2d4b-5a9e-8c0a-3b4c5d6e7f8g)

Deleting this repository will also delete these subscriptions and their updates.
Do you want to proceed? (y/N): y

Repository rust-lang/rust deleted along with 2 subscriptions and 45 updates.
```

### Working with Multiple Repositories

```
# Add multiple repositories
xiaotian> add repository microsoft vscode
Repository added: microsoft/vscode

xiaotian> add repository facebook react
Repository added: facebook/react

# Add subscriptions with different update frequencies
xiaotian> add subscription microsoft vscode
Subscription added for: microsoft/vscode (7a8b9c0d-1e2f-3g4h-5i6j-7k8l9m0n1o2p)

xiaotian> add subscription facebook react
Subscription added for: facebook/react (3e4f5g6h-7i8j-9k0l-1m2n-3o4p5q6r7s8t)

# List all repositories
xiaotian> list repositories
Repositories:
- microsoft/vscode (Last updated: 2025-04-12)
- facebook/react (Last updated: 2025-04-11)
- rust-lang/rust (Last updated: 2025-04-10)

# Fetch updates for a specific subscription
xiaotian> fetch updates 7a8b9c0d-1e2f-3g4h-5i6j-7k8l9m0n1o2p 3
Found 28 updates for microsoft/vscode...
```

## Development

XiaoTian is currently in version 0.2.5. Key features implemented:

- Core architecture and storage system (v0.1.0)
- GitHub API integration (v0.2.0)
- REPL commands for configuration and update management (v0.2.1)
- Command module refactoring and improved error handling (v0.2.2)
- Command line parsing with Clap integration (v0.2.3)
- Enhanced REPL experience with reedline-repl-rs (v0.2.4)
- Data relationship integrity and enhanced user experience (v0.2.5)

### New in v0.2.5: Data Relationship & User Experience

Version 0.2.5 focuses on improving data relationship integrity, user interaction, and code quality:

- Reference integrity checks when adding subscriptions verify that the corresponding source exists
- Interactive confirmation workflow for destructive operations
- Improved error messages when attempting operations that would violate data integrity
- Code optimizations to reduce duplication and increase maintainability

Example of the new interactive confirmation flow when deleting a repository:

```
xiaotian> delete repository rust-lang rust
This repository has 2 related subscriptions:
- rust-lang/rust (8f3e8b42-1c3d-4a8e-9b9a-2b3c4d5e6f7g)
- rust-lang/rust [Weekly] (9e2f8c53-2d4b-5a9e-8c0a-3b4c5d6e7f8g)

Deleting this repository will also delete these subscriptions and their updates.
Do you want to proceed? (y/N): y

Repository rust-lang/rust deleted along with 2 subscriptions and 45 updates.
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

## Contributing

Contributions to XiaoTian are welcome! Here's how you can contribute:

### Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Commit your changes: `git commit -m 'Add some amazing feature'`
5. Push to the branch: `git push origin feature/amazing-feature`
6. Open a Pull Request

### Development Guidelines

- Follow the Rust style guide
- Write tests for new features
- Update documentation as needed
- Add entries to the changelog for notable changes

### Setting Up Development Environment

```bash
# Clone your fork
git clone https://github.com/your-username/xiaotian.git
cd xiaotian

# Add the original repository as a remote
git remote add upstream https://github.com/hedon954/xiaotian.git

# Create a feature branch
git checkout -b feature/your-feature-name

# Install development dependencies
cargo install --path .
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [Octocrab](https://github.com/XAMPPRocky/octocrab) for GitHub API integration
- [Clap](https://github.com/clap-rs/clap) for command-line argument parsing
- [reedline-repl-rs](https://github.com/kurtlawrence/reedline-repl-rs) for the interactive REPL interface
- All contributors who have helped shape and improve this project
