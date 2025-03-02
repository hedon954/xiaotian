use std::sync::Arc;

use colored::Colorize;
use rustyline::{DefaultEditor, Result as RustylineResult};
use tokio::runtime::Handle;

use super::commands::CommandHandler;
use crate::storage::Storage;

/// REPL (Read-Eval-Print Loop) for XiaoTian
pub struct Repl<S: Storage> {
    command_handler: CommandHandler<S>,
    editor: DefaultEditor,
    runtime_handle: Handle,
}

impl<S: Storage> Repl<S> {
    /// Create a new REPL
    pub fn new(storage: Arc<S>, runtime_handle: Handle) -> RustylineResult<Self> {
        let command_handler = CommandHandler::new(storage);
        let editor = DefaultEditor::new()?;

        Ok(Self {
            command_handler,
            editor,
            runtime_handle,
        })
    }

    /// Start the REPL
    pub fn start(&mut self) -> RustylineResult<()> {
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

                    if line.trim() == "exit" || line.trim() == "quit" {
                        println!("Goodbye!");
                        break;
                    }

                    self.process_command(&line);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Process a command
    fn process_command(&self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        let command = parts[0];
        let args = if parts.len() > 1 {
            parts[1..].to_vec()
        } else {
            Vec::new()
        };

        // Execute the command in the tokio runtime
        let result = self
            .runtime_handle
            .block_on(async { self.command_handler.execute(command, args).await });

        match result {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}: {}", "Error".bright_red(), err),
        }
    }
}
