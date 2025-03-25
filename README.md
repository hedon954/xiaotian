# XiaoTian (哮天犬)

```
                 .--~~,__
    :-....,-------`~~'._.\'
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
- AI-powered update summaries with LLM integration (Ollama)
- Email notification system for repository updates
- Customizable update display with filtering options
- Interactive confirmation for destructive operations
- User-friendly REPL interface with history support
- Scheduled updates and report generation

## Installation

### Prerequisites

- Rust 1.70 or higher (for building from source)
- GitHub Personal Access Token (optional, for higher API rate limits)
- Ollama service (optional, for AI-powered summaries)
- SMTP server access (optional, for email notifications)

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
XiaoTian 0.5.0
```

## Configuration

### Basic Configuration

Create a `config.toml` file in your XiaoTian configuration directory:

```toml
[github]
token = "your-github-token"

[notification]
enabled = true
default_channel = "email"

[notification.email]
smtp_server = "smtp.example.com"
smtp_port = 587
username = "your-email@example.com"
password = "your-password"
from = "XiaoTian <your-email@example.com>"
to = ["recipient@example.com"]
use_tls = true
```

## Usage

```bash
Commands available in XiaoTian

Usage: xiaotian [COMMAND]

Commands:
  add     Add a repository or subscription
  list    List repositories or subscriptions
  delete  Delete a repository or subscription
  fetch   Fetch updates for subscriptions
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Command Structure

XiaoTian uses a hierarchical command structure with the following main commands:

### Repository Management

- `add repo <owner> <name>` - Add a GitHub repository
- `list repos` - List all repositories
- `delete repo <id>` - Delete a repository (with interactive confirmation)

### Update Operations

- `fetch updates <repository_id> [days]` - Fetch updates for a repository for the last N days

## Development

XiaoTian is currently in version 0.6.0. Key features implemented:

- Core architecture and storage system (v0.1.0)
- GitHub API integration (v0.2.0)
- REPL commands for configuration and update management (v0.2.1)
- Command module refactoring and improved error handling (v0.2.2)
- Command line parsing with Clap integration (v0.2.3)
- Enhanced REPL experience with reedline-repl-rs (v0.2.4)
- Data relationship integrity and enhanced user experience (v0.2.5)
- ID system simplification with intuitive integer IDs (v0.2.6)
- Architecture simplification with streamlined data flow (v0.2.7)
- Scheduled updates and structured report generation (v0.3.0)
- LLM integration and AI-powered update summaries (v0.4.0)
- Email notification system for updates (v0.5.0)
- Gradio web interface for easy interaction (v0.6.0)

### New in v0.6.0: Gradio Web Interface

Version 0.6.0 introduces a user-friendly web interface using Gradio:

- Interactive web UI for repository updates and summaries
- Model selection for different LLM versions
- Source type and repository input
- Markdown display for original and generated content
- Email notification configuration
- Download and send functionality

Example usage of the Gradio interface:

```bash
# Navigate to the frontend directory
cd frontend-gradio

# Install dependencies
pip install -r requirements.txt

# Start the Gradio interface
python src/app.py
```

The web interface will be available at `http://localhost:7860` by default.

## Roadmap

### Coming Soon in v0.6.0: Testing & Documentation

- Comprehensive integration testing
- End-to-end testing
- Documentation improvements
- Code coverage enhancements
- Performance benchmarks

### Future Versions

- v0.7.0: Enhanced notifications (Slack, Discord)
- v0.8.0: Persistent storage and optimization
- v1.0.0: Stable release with all MVP features

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [Octocrab](https://github.com/XAMPPRocky/octocrab) for GitHub API integration
- [Clap](https://github.com/clap-rs/clap) for command-line argument parsing
- [reedline-repl-rs](https://github.com/kurtlawrence/reedline-repl-rs) for the interactive REPL interface
- [Ollama](https://github.com/ollama/ollama) for local LLM integration
- [mail-send](https://github.com/stalwartlabs/mail-send) for SMTP email support
- All contributors who have helped shape and improve this project
