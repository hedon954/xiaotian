//! Help command handler

/// Handler for the help command
pub struct HelpHandler;

impl HelpHandler {
    /// Handle the help command
    pub fn handle() -> String {
        r#"
XiaoTian - GitHub Repository Tracker

Available commands:
  help                                Show this help message
  add repo <owner> <name>             Add a repository
  add subscription <owner> <name>     Subscribe to a repository
  list repos                          List all repositories
  list subscriptions                  List all subscriptions
  list updates                        List all updates
  show repo <owner> <name>            Show repository details
  show subscription <id>              Show subscription details
  show updates <subscription_id> [n]  Show updates for subscription (optional limit)
  delete repo <id>                    Delete a repository
  delete subscription <id>            Delete a subscription
  clear                               Clear all data
  fetch updates <subscription_id> [days]  Fetch updates for a subscription (optional days)
  exit, quit                          Exit the application
"#
        .to_string()
    }
}
