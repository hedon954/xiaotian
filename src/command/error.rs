//! Error types for command module

use thiserror::Error;

/// Errors that can occur during command parsing and validation
#[derive(Debug, Error)]
pub enum CommandError {
    /// Command is empty
    #[error("Command is empty")]
    EmptyCommand,

    /// Command is not recognized
    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    /// Command arguments are invalid
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CommandError::EmptyCommand;
        assert_eq!(err.to_string(), "Command is empty");

        let err = CommandError::UnknownCommand("test".to_string());
        assert_eq!(err.to_string(), "Unknown command: test");

        let err = CommandError::InvalidArguments("missing args".to_string());
        assert_eq!(err.to_string(), "Invalid arguments: missing args");
    }
}
