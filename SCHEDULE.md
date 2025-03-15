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
| v0.4.0    | LLM Integration & Summaries   | - Create LLM utility for AI-powered content generation<br>- Implement Ollama integration<br>- Generate update summaries using LLM<br>- Create enhanced report format with AI summaries                                 | - LLM integration framework<br>- AI-powered update summaries<br>- Enhanced report quality<br>- Improved information density   |
| v0.5.0    | Notification System           | - Implement notification system<br>- Add report management commands<br>- Support multiple notification channels<br>- Add delivery mechanisms for reports                                                               | - Notification system<br>- Report delivery<br>- Multiple notification channels                                                |
| v0.6.0    | MVP Release                   | - Integration testing<br>- Bug fixes<br>- Documentation<br>- User experience improvements                                                                                                                              | - MVP release<br>- User documentation<br>- Developer documentation                                                            |
| v0.7.0    | Enhanced Reports              | - Support multiple report formats (text, HTML, PDF)<br>- Add persistent storage support<br>- Implement report templates<br>- Add custom report configuration                                                           | - Multi-format reports<br>- Template system<br>- Persistent storage<br>- Customization options                                |
| v0.8.0    | Storage & Optimization        | - Implement persistent storage backends<br>- Performance optimizations<br>- Resource usage improvements<br>- Configuration management                                                                                  | - Database storage implementation<br>- Optimized performance<br>- Advanced configuration options                              |

### Stable Release (v1.0.0)

Initial stable release with all MVP features properly tested and documented.

### Future Versions

| Version | Feature Category | Potential Features                                                                                                      |
| ------- | ---------------- | ----------------------------------------------------------------------------------------------------------------------- |
| v1.1.0  | Authorization    | - GitHub authentication<br>- Access token management<br>- Rate limit handling                                           |
| v1.2.0  | User Interface   | - Web interface integration<br>- Dashboard for visualizing updates                                                      |
| v1.3.0  | Integration      | - Additional notification channels (Email, Slack, Discord)<br>- Multi-user support<br>- API for third-party integration |

## MVP Core Features (v0.5.0)

1. **Repository Management**

   - Add/remove GitHub repository sources
   - List and filter repositories
   - Group repositories by tags/categories

2. **Update Retrieval**

   - Fetch updates from GitHub (commits, PRs, issues, releases)
   - Schedule regular update checks
   - Process updates with source-specific logic

3. **Notification System**

   - Generate notifications for new updates
   - Basic notification rules

4. **Report Generation**
   - Generate daily/weekly summaries
   - Support text and markdown formats
   - Customizable report content
