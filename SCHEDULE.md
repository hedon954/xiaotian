# XiaoTian Project Schedule

## Project Overview

XiaoTian is a command-line tool designed for developers and project managers to automatically track and summarize updates from subscribed GitHub repositories. The tool provides repository subscription management, update retrieval, notification system, and report generation.

## Development Roadmap

### Pre-Release Versions (v0.x.x)

| Version | Focus                        | Tasks                                                                                                                                                                              | Deliverables                                                                                                                  |
| ------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| v0.1.0  | Project Setup & Architecture | - Initialize project structure<br>- Set up development environment<br>- Design data models<br>- Define storage traits with memory implementation<br>- Create basic REPL interface  | - Project skeleton<br>- Basic REPL CLI<br>- Storage trait interfaces<br>- In-memory storage implementation                    |
| v0.2.0  | GitHub API Integration       | - Implement GitHub API client<br>- Develop repository data fetching<br>- Create repository subscription management<br>- Support basic CRUD for subscriptions                       | - GitHub API client<br>- Data models for repositories<br>- Subscription management commands<br>- Data persistence (in-memory) |
| v0.2.1  | Command Enhancement          | - Add fetch updates command<br>- Add config command for GitHub API token<br>- Extend show command for subscription updates<br>- Implement configuration management                 | - GitHub API token management<br>- Update fetching commands<br>- Configuration system<br>- Enhanced display features          |
| v0.2.2  | Command Module Refactoring   | - Split command parsing into separate module<br>- Split command execution into process module<br>- Implement specialized command handlers<br>- Unify error handling<br>- Add tests | - Decoupled command architecture<br>- Improved command extensibility<br>- Better error handling<br>- Increased test coverage  |
| v0.2.3  | Clap Integration             | - Add Clap library integration<br>- Create derive-based command structs<br>- Build hierarchical subcommand system<br>- Improve UI and error messaging                              | - Robust command parsing<br>- Better help documentation<br>- Improved user feedback<br>- Support for CLI and REPL modes       |
| v0.2.4  | REPL & Architecture Upgrade  | - Replace custom REPL with reedline-repl-rs<br>- Create configuration management system<br>- Optimize data models and architecture<br>- Improve performance and storage            | - Enhanced user experience<br>- Improved architecture<br>- Better performance<br>- Foundation for persistent storage          |
| v0.3.0  | Update Tracking              | - Implement update retrieval logic<br>- Develop incremental update mechanism<br>- Create update storage and querying<br>- Add scheduled update checks                              | - Update tracking system<br>- Incremental update logic<br>- Scheduled task framework                                          |
| v0.4.0  | Notification & Reports       | - Implement basic notification system<br>- Develop report generation<br>- Support multiple report formats (text, markdown)<br>- Add filtering capabilities                         | - Notification system<br>- Report generation<br>- Multiple output formats                                                     |
| v0.5.0  | MVP Release                  | - Integration testing<br>- Bug fixes<br>- Documentation<br>- User experience improvements                                                                                          | - MVP release<br>- User documentation<br>- Developer documentation                                                            |
| v0.6.0  | Enhanced Features            | - Enhanced notification options<br>- Advanced filtering and querying<br>- Improved report customization<br>- Additional GitHub event types                                         | - Enhanced notification system<br>- Advanced filters<br>- Customizable reports                                                |
| v0.8.0  | Storage & Optimization       | - Implement persistent storage backends<br>- Performance optimizations<br>- Resource usage improvements<br>- Configuration management                                              | - Database storage implementation<br>- Optimized performance<br>- Advanced configuration options                              |

### Stable Release (v1.0.0)

Initial stable release with all MVP features properly tested and documented.

### Future Versions

| Version | Feature Category | Potential Features                                                                                                      |
| ------- | ---------------- | ----------------------------------------------------------------------------------------------------------------------- |
| v1.1.0  | Authorization    | - GitHub authentication<br>- Access token management<br>- Rate limit handling                                           |
| v1.2.0  | User Interface   | - Web interface integration<br>- Dashboard for visualizing updates                                                      |
| v1.3.0  | Integration      | - Additional notification channels (Email, Slack, Discord)<br>- Multi-user support<br>- API for third-party integration |

## MVP Core Features (v0.5.0)

1. **Repository Subscription Management**

   - Add/remove GitHub repository subscriptions
   - List and filter subscriptions
   - Group repositories by tags/categories

2. **Update Retrieval**

   - Fetch updates from GitHub (commits, PRs, issues, releases)
   - Schedule regular update checks
   - Track and store update history

3. **Notification System**

   - Generate notifications for new updates
   - Basic notification rules

4. **Report Generation**
   - Generate daily/weekly summaries
   - Support text and markdown formats
   - Customizable report content
