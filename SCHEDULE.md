# XiaoTian Project Schedule

## Project Overview

XiaoTian is a command-line tool designed for developers and project managers to automatically track and summarize updates from subscribed GitHub repositories. The tool provides repository subscription management, update retrieval, notification system, and report generation.

## Development Roadmap

### Pre-Release Versions (v0.x.x)

| Version   | Focus                         | Tasks                                                                                                                                                                                                                  | Deliverables                                                                                                                  |
| --------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| v0.1.0    | Project Setup & Architecture  | - Initialize project structure<br>- Set up development environment<br>- Design data models<br>- Define storage traits with memory implementation<br>- Create basic REPL interface                                      | - Project skeleton<br>- Basic REPL CLI<br>- Storage trait interfaces<br>- In-memory storage implementation                    |
| v0.2.0    | GitHub API Integration        | - Implement GitHub API client<br>- Develop repository data fetching<br>- Create repository subscription management<br>- Support basic CRUD for subscriptions                                                           | - GitHub API client<br>- Data models for repositories<br>- Subscription management commands<br>- Data persistence (in-memory) |
| v0.2.1    | Command Enhancement           | - Add fetch updates command<br>- Add config command for GitHub API token<br>- Extend show command for subscription updates<br>- Implement configuration management                                                     | - GitHub API token management<br>- Update fetching commands<br>- Configuration system<br>- Enhanced display features          |
| v0.2.2    | Command Module Refactoring    | - Split command parsing into separate module<br>- Split command execution into process module<br>- Implement specialized command handlers<br>- Unify error handling<br>- Add tests                                     | - Decoupled command architecture<br>- Improved command extensibility<br>- Better error handling<br>- Increased test coverage  |
| v0.2.3    | Clap Integration              | - Add Clap library integration<br>- Create derive-based command structs<br>- Build hierarchical subcommand system<br>- Improve UI and error messaging                                                                  | - Robust command parsing<br>- Better help documentation<br>- Improved user feedback<br>- Support for CLI and REPL modes       |
| v0.2.4    | REPL & Architecture Upgrade   | - Replace custom REPL with reedline-repl-rs<br>- Create configuration management system<br>- Optimize data models and architecture<br>- Improve performance and storage                                                | - Enhanced user experience<br>- Improved architecture<br>- Better performance<br>- Foundation for persistent storage          |
| v0.2.5    | Data Relationship & Integrity | - Implement reference integrity between models<br>- Add cascade delete operations<br>- Improve user confirmation for destructive actions<br>- Enhance error handling for relationships                                 | - Robust data consistency<br>- Safer delete operations<br>- Better user guidance<br>- Improved error messages                 |
| v0.2.6    | ID System Simplification      | - Replace UUID with auto-increment integers<br>- Optimize model relationships with direct ID references<br>- Improve user experience with simpler IDs<br>- Enhance error handling                                      | - Simplified ID system<br>- More intuitive CLI interactions<br>- Reduced complexity<br>- Better error context                 |
| v0.2.7    | Architecture Simplification   | - Remove subscription concept, use direct source management<br>- Eliminate Update storage, process updates on-demand<br>- Remove related commands for streamlined interface<br>- Simplify command naming for better UX | - Streamlined data model<br>- Simplified command interface<br>- Reduced complexity<br>- More focused architecture             |
| v0.3.0 ✅ | Scheduled Updates & Reports   | - Implement scheduled tasks using cron_tab<br>- Generate and store markdown reports for update results<br>- Create organized report directory structure<br>- Optimize report content focused on high-value updates     | - Improved code architecture<br>- Automated update scheduling<br>- Persistent report generation<br>- Enhanced update tracking |
| v0.4.0 ✅ | LLM Integration & Summaries   | - Create LLM utility for AI-powered content generation<br>- Implement Ollama integration<br>- Generate update summaries using LLM<br>- Create enhanced report format with AI summaries                                 | - LLM integration framework<br>- AI-powered update summaries<br>- Enhanced report quality<br>- Improved information density   |
| v0.5.0 ✅ | Notification System           | - Implement notification system<br>- Add report management commands<br>- Support multiple notification channels<br>- Add delivery mechanisms for reports                                                               | - Notification system<br>- Report delivery<br>- Multiple notification channels                                                |
| v0.6.0 ✅ | Gradio Web Interface          | - Create frontend-gradio directory<br>- Implement Gradio UI components<br>- Add placeholder functions<br>- Set up Python environment<br>- Create mock data for testing                                                 | - Web interface with Gradio<br>- Python environment setup<br>- Mock functionality<br>- Basic UI testing                       |
| v0.6.1 ✅ | Python-Rust FFI Integration   | - Implement PyO3 bindings for core functions<br>- Bridge Gradio UI with Rust backend<br>- Add data converters between Rust and Python<br>- Replace mock functions with real calls                                      | - Python-Rust FFI<br>- Live data in UI<br>- Email notification<br>- Real functionality                                        |
| v0.7.0 ✅ | Advanced UI & FFI Features    | - Implement PyO3 bindings for advanced features<br>- Create Python package structure<br>- Improve UI responsiveness<br>- Add error handling<br>- Integration testing                                                   | - Complete Python-Rust integration<br>- Full UI functionality<br>- Error handling<br>- Integration tests                      |
| v0.8.1 ✅ | Multiple Data Sources         | - Design unified source interface<br>- Implement HackerNews data source<br>- Support multiple feed types<br>- Create uniform data model<br>- Enhance reporting capabilities                                            | - Unified source interface<br>- HackerNews integration<br>- Multiple feed types<br>- Enhanced reports                         |
| v0.8.2 ✅ | Multiple LLM Providers        | - Create unified LLM client interface<br>- Add DeepSeek integration<br>- Support hot-swapping LLM providers<br>- Configure different models<br>- Optimize prompt engineering                                           | - LLM Provider abstraction<br>- DeepSeek integration<br>- Model configuration<br>- Enhanced prompt system                     |
| v0.8.3 ✅ | MySQL Storage Support         | - Design flexible storage interface<br>- Implement MySQL backend<br>- Optimize with JSON field usage<br>- Create migration utilities<br>- Add transaction support                                                      | - Storage abstraction layer<br>- MySQL integration<br>- JSON field optimization<br>- Data migration tools                     |
| v0.9.0    | Integration & Testing         | - System integration testing<br>- End-to-end testing<br>- Performance benchmarks<br>- Code quality improvements<br>- Documentation updates                                                                             | - Test suite<br>- Performance metrics<br>- Code quality enhancements<br>- Comprehensive documentation                         |

### Future Releases (v0.9.x and beyond)

| Version | Focus                  | Features                                                                                                                                                                                              |
| ------- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| v0.9.1  | RSS Feed Integration   | - Generic RSS feed support<br>- Feed discovery and extraction<br>- Content categorization<br>- Flexible feed configuration                                                                            |
| v0.9.2  | Enhanced Notifications | - Slack integration<br>- Discord webhooks<br>- Custom notification templates<br>- Notification scheduling and batching                                                                                |
| v0.9.3  | Authentication & Users | - User authentication system<br>- Multi-user support<br>- Permission management<br>- User preferences                                                                                                 |
| v1.0.0  | Production Release     | - Comprehensive documentation<br>- Performance optimization<br>- Installation packages<br>- CI/CD pipeline<br>- Docker support                                                                        |
| v1.1.0  | Analytics & Metrics    | - Update statistics and trends<br>- User engagement metrics<br>- System health monitoring<br>- Data visualization<br>- Interactive dashboards                                                         |
| v1.2.0  | API & Integration      | - REST API for external access<br>- Integration with third-party services<br>- Plugin system<br>- Custom extension points<br>- WebHook support                                                        |
| v1.3.0  | Advanced Data Sources  | - Twitter/X integration<br>- Reddit tracking<br>- StackOverflow monitoring<br>- Custom API integration framework<br>- Data source marketplace                                                         |
| v1.4.0  | Enhanced AI Features   | - Multi-modal content analysis (images, code)<br>- Context-aware summaries<br>- Historical trend analysis<br>- Custom LLM fine-tuning<br>- Vector database integration for semantic search and recall |

## Core Features

### Data Sources

- ✅ GitHub Repositories (commits, issues, PRs, releases)
- ✅ HackerNews (frontpage, newest, best, ask, show, jobs)
- 🔄 Generic RSS feeds
- 🔄 Twitter/X
- 🔄 Reddit
- 🔄 StackOverflow

### LLM Providers

- ✅ Ollama (local deployment)
- ✅ DeepSeek (cloud-based)
- 🔄 OpenAI
- 🔄 Anthropic Claude
- 🔄 Gemini

### Storage Engines

- ✅ In-memory (ephemeral)
- ✅ MySQL (persistent)
- 🔄 PostgreSQL
- 🔄 SQLite
- 🔄 MongoDB

### Interfaces

- ✅ Command-line (CLI)
- ✅ Interactive shell (REPL)
- ✅ Web interface (Gradio)
- 🔄 REST API
- 🔄 Mobile application

### Notification Channels

- ✅ Email
- 🔄 Slack
- 🔄 Discord
- 🔄 Telegram
- 🔄 SMS

✅ = Implemented
🔄 = Planned
