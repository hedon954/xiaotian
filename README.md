# XiaoTian (哮天犬)

```
                 .--~~,__
    :-....,-------`~~'._.\'
    `-,,,  ,_      ;'~U'
     _,-' ,'`-__; '--.     (XiaoTian)
    (_/'~~      ''''(;
```

> XiaoTian (哮天犬) is named after the legendary "Howling Celestial Dog" in Chinese mythology, known for its keen sense of smell and ability to track down anything. Just like its namesake, this tool is designed to "sniff out" the latest updates from various sources and generate AI-powered summaries.

XiaoTian is a flexible monitoring system designed to track, summarize, and notify about updates from multiple data sources. With a modular architecture, it supports various data sources, storage engines, and LLM providers. Interact through a web interface powered by Gradio, REPL, or CRON.

## Key Features

- **Multiple Data Sources**: GitHub repositories, HackerNews, and more
- **Multiple LLM Integrations**: Ollama, DeepSeek, and others
- **Flexible Storage Options**: In-memory, MySQL, and expandable
- **Multiple Interfaces**: Web UI (Gradio), REPL, CRON
- **Notification System**: Email alerts with rich, AI-powered summaries
- **Modular Architecture**: Easily extend with new data sources, storage engines, or LLM providers

## Requirements

- Python 3.10+
- Rust 1.70+
- maturin
- MySQL 8.0+(optional)
- Ollama(optional)
- DeepSeek API KEY(optional)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/hedon954/xiaotian.git
cd xiaotian
```

### Use REPL

```bash
cd xiaotian
cargo run --bin xiaotian help
cargo build --bin xiaotian --release
```

### Run in Cron

```bash
cd xiaotian
cargo run --bin cron
```

### Use Gradio

```bash
# Build xiaotian-py-binding
cd xiaotian-py-binding
maturin develop

# Start the web interface
cd .. && cd frontend-gradio
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python run.py
```

## Configuration

Create a `config.toml` file:

```toml
[github]
token = "your-github-token"

[notification]
enabled = true
default_channel = "email"

[notification.email]
smtp_server = "smtp.qq.com"
smtp_port = 465
username = "your-email@qq.com"
password = "your-password"
from = "your-email@qq.com"
to = ["your-email@qq.com"]
use_tls = true

[deepseek]
api_key = "sk-xxxxxx"
api_base = "https://api.deepseek.com/v1"
model = "deepseek-chat"

[storage]
type = "MySQL" # Options: "Memory", "MySQL"

[storage.mysql]
dsn = "mysql://root:root@localhost:3306/xiaotian"
```

## Supported Data Sources

- **GitHub**: Monitor repositories for commits, issues, PRs and more
- **HackerNews**: Track top stories, newest, Ask HN, Show HN
- More sources coming soon!

## Supported LLM Providers

- **Ollama**: Local deployment with various open-source models
- **DeepSeek**: Cloud-based AI with powerful capabilities
- Support for more providers planned

## Storage Options

- **Memory**: Fast, ephemeral storage for testing and demo purposes
- **MySQL**: Persistent storage with JSON data type support
- More storage engines planned (PostgreSQL, SQLite)

## Interfaces

1. **Web UI**: Intuitive Gradio interface at http://localhost:7860
2. **CRON**: Run XiaoTian in the background to periodically check for updates
3. **REPL**: Interactive shell for exploration and debugging

## License

[MIT License](LICENSE)

## Acknowledgements

- [Octocrab](https://github.com/XAMPPRocky/octocrab) for GitHub API
- [Ollama](https://github.com/ollama/ollama) and [DeepSeek](https://deepseek.com) for LLM support
- [PyO3](https://github.com/PyO3/pyo3) for Rust-Python bindings
- [Gradio](https://github.com/gradio-app/gradio) for web UI
- [SQLx](https://github.com/launchbadge/sqlx) for database access
