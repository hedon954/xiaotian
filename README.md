# XiaoTian (哮天犬)

```
                 .--~~,__
    :-....,-------`~~'._.\'
    `-,,,  ,_      ;'~U'
     _,-' ,'`-__; '--.     (XiaoTian)
    (_/'~~      ''''(;
```

> XiaoTian (哮天犬) is named after the legendary "Howling Celestial Dog" in Chinese mythology, known for its keen sense of smell and ability to track down anything. Just like its namesake, this tool is designed to "sniff out" the latest updates from GitHub repositories you care about and generate AI-powered summaries.

XiaoTian is a tool designed for developers and project managers to automatically track, summarize, and notify about updates from GitHub repositories. It features an intuitive web interface powered by Gradio and AI-driven summaries using LLM technology.

## Features

- **Repository Tracking**: Monitor GitHub repositories for updates and changes
- **AI-Powered Summaries**: Generate concise summaries using LLM integration (Ollama)
- **Web Interface**: User-friendly Gradio UI for easy interaction
- **Email Notifications**: Receive update reports directly in your inbox
- **Python Integration**: Rust core with Python bindings for maximum flexibility
- **Report Generation**: Structured Markdown reports for organized information

## Quick Start

### Option 1: Using the Web Interface (Recommended)

1. Make sure you have Python and Rust installed
2. Clone the repository:
   ```bash
   git clone https://github.com/hedon954/xiaotian.git
   cd xiaotian
   ```
3. Launch the web interface:
   ```bash
   cd frontend-gradio
   pip install -r requirements.txt
   python run.py
   ```
4. Open your browser at http://localhost:7860

### Option 2: Using the Command Line

```bash
# Clone the repository
git clone https://github.com/hedon954/xiaotian.git
cd xiaotian

# Build the project
cargo build --release

# Run the scheduled update service
cargo run --bin cron

# View generated reports in the reports directory
```

## System Requirements

- **Rust** 1.70 or higher
- **Python** 3.8 or higher (for web interface)
- **Ollama** set up and running on localhost:11434 (for AI summaries)
- **SMTP Server** access (for email notifications)

## Configuration

Create a `config.toml` file in the project root with the following settings:

```toml
[github]
token = "your-github-token"  # Optional, for higher API rate limits

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

## Architecture

XiaoTian is built with a layered architecture:

1. **Core Engine (Rust)**: High-performance data fetching and processing
2. **LLM Integration**: AI-powered summary generation
3. **Notification System**: Email delivery of reports
4. **Python Bindings**: PyO3-based FFI for language interoperability
5. **Web Interface**: Gradio-based UI for easy interaction

## Development Roadmap

### Current Version (v0.7.0)

- ✅ Simplified architecture focused on core functionality
- ✅ Python FFI bindings with PyO3
- ✅ Gradio web interface integration
- ✅ Email notification system
- ✅ Enhanced LLM integration with Ollama

### Upcoming in v0.8.x

- **v0.8.1**: MySQL storage support for persistent data
- **v0.8.2**: DeepSeek LLM integration
- **v0.8.3**: HackerNews source support

### Future Plans (v0.9.0+)

- Slack and Discord notification channels
- Additional content sources (GitLab, Bitbucket)
- Custom report templates
- User authentication system
- Analytics and visualization

## Web Interface Guide

The Gradio web interface provides an intuitive way to interact with XiaoTian:

1. **Select Repository**: Choose a GitHub repository to track
2. **Choose LLM Model**: Select your preferred language model
3. **Enter Email**: Optional, for receiving notifications
4. **Generate Report**: Click to fetch updates and generate summaries
5. **View Results**: See original content and AI-generated summaries side by side
6. **Download or Send**: Save the report or send it via email

## Command Line Usage

For advanced users, XiaoTian provides a powerful command-line interface:

```bash
# Generate reports for all repositories
cargo run --bin cron

# Add a new repository to track
cargo run --bin xiaotian add repo owner-name repo-name

# List all tracked repositories
cargo run --bin xiaotian list repos

# Delete a repository by ID
cargo run --bin xiaotian delete repo <id>
```

## Contributing

Contributions are welcome! Please check the [issues](https://github.com/hedon954/xiaotian/issues) page for current needs.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Octocrab](https://github.com/XAMPPRocky/octocrab) for GitHub API integration
- [Ollama](https://github.com/ollama/ollama) for local LLM support
- [PyO3](https://github.com/PyO3/pyo3) for Rust-Python FFI
- [Gradio](https://github.com/gradio-app/gradio) for the web interface
- [mail-send](https://github.com/stalwartlabs/mail-send) for SMTP email support
