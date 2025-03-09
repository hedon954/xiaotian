use std::sync::Arc;

use colored::Colorize;
use rustyline::{DefaultEditor, Result as RustylineResult};

use crate::command::{Command, CommandError, CommandParser};
use crate::models::source::SourceFactory;
use crate::process::{CommandProcessor, CommandProcessorImpl, ProcessError};
use crate::storage::Storage;

/// REPL (Read-Eval-Print Loop) for XiaoTian
pub struct Repl<S: Storage + Send + Sync + 'static> {
    command_processor: CommandProcessorImpl<S>,
    editor: DefaultEditor,
}

impl<S: Storage + Send + Sync + 'static> Repl<S> {
    /// Create a new REPL
    pub fn new(storage: Arc<S>, source_factory: Arc<dyn SourceFactory>) -> RustylineResult<Self> {
        let command_processor = CommandProcessorImpl::new(storage, source_factory);
        let editor = DefaultEditor::new()?;

        Ok(Self {
            command_processor,
            editor,
        })
    }

    /// Start the REPL
    pub async fn start(&mut self) -> RustylineResult<()> {
        println!(
            "{}",
            "Welcome to XiaoTian - GitHub Repository Tracker".bright_green()
        );
        println!("Type {} for available commands", "help".bright_yellow());

        loop {
            let readline = self.editor.readline("xiaotian> ");
            match readline {
                Ok(line) => {
                    self.editor.add_history_entry(&line)?;

                    let trimmed = line.trim();
                    if trimmed == "exit" || trimmed == "quit" {
                        println!("{}", "Goodbye!".bright_green());
                        break;
                    }

                    // Skip empty lines
                    if trimmed.is_empty() {
                        continue;
                    }

                    self.process_command(trimmed).await;
                }
                Err(err) => {
                    eprintln!("{}: {:?}", "Error reading input".bright_red(), err);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Process a command
    async fn process_command(&self, line: &str) {
        // Parse the command
        let command = match CommandParser::parse(line) {
            Ok(cmd) => cmd,
            Err(err) => {
                self.print_command_error(&err, line);
                return;
            }
        };

        // Check for exit command (already handled in the loop)
        if matches!(command, Command::Exit) {
            return;
        }

        // Execute the command
        let result = self.command_processor.process(command).await;

        match result {
            Ok(output) => println!("{}", output),
            Err(err) => self.print_process_error(&err),
        }
    }

    /// Print a command error with helpful suggestions
    fn print_command_error(&self, err: &CommandError, input: &str) {
        eprintln!("{}: {}", "Command Error".bright_red(), err);

        // Provide suggestions based on error type
        match err {
            CommandError::UnknownCommand(cmd) => {
                eprintln!(
                    "{}",
                    format!(
                        "Unknown command: '{}'. Type 'help' to see available commands.",
                        cmd
                    )
                    .yellow()
                );
            }
            CommandError::InvalidArguments(args_err) => {
                eprintln!(
                    "{}",
                    format!(
                        "Invalid arguments: {}. Type 'help' for usage examples.",
                        args_err
                    )
                    .yellow()
                );
            }
            CommandError::EmptyCommand => {
                eprintln!(
                    "{}",
                    "Please enter a command. Type 'help' to see available commands.".yellow()
                );
            }
        }

        // Try to suggest similar commands for unknown input
        if let CommandError::UnknownCommand(_) = err {
            let first_word = input.split_whitespace().next().unwrap_or("");
            let suggestions = self.get_command_suggestions(first_word);
            if !suggestions.is_empty() {
                eprintln!(
                    "{}: {}",
                    "Did you mean".bright_yellow(),
                    suggestions.join(", ")
                );
            }
        }
    }

    /// Print a process error with context
    fn print_process_error(&self, err: &ProcessError) {
        eprintln!("{}: {}", "Error".bright_red(), err);

        // Add more context based on error type
        match err {
            ProcessError::NotFound(msg) => {
                eprintln!(
                    "{}",
                    format!("{}. Use 'list' commands to see available items.", msg).yellow()
                );
            }
            ProcessError::General(msg) => {
                eprintln!(
                    "{}",
                    format!("{}. Please check your input and try again.", msg).yellow()
                );
            }
            ProcessError::SourceError(err) => {
                eprintln!(
                    "{}",
                    format!("Error communicating with source: {}. Check your network connection and GitHub token.", err)
                        .yellow()
                );
            }
            ProcessError::StorageError(err) => {
                eprintln!(
                    "{}",
                    format!(
                        "Storage error: {}. This might be a bug in the application.",
                        err
                    )
                    .yellow()
                );
            }
            _ => {}
        }
    }

    /// Get command suggestions for a given input
    fn get_command_suggestions(&self, input: &str) -> Vec<String> {
        let commands = [
            "help", "add", "list", "show", "delete", "clear", "fetch", "exit",
        ];

        commands
            .iter()
            .filter(|&cmd| cmd.starts_with(input) && *cmd != input)
            .map(|&cmd| cmd.to_string())
            .collect()
    }
}
