use std::sync::Arc;

use colored::Colorize;
use rustyline::{DefaultEditor, Result as RustylineResult};
use tokio::runtime::Handle;

use super::commands::CommandHandler;
use crate::models::source::SourceFactory;
use crate::storage::Storage;

/// REPL (Read-Eval-Print Loop) for XiaoTian
pub struct Repl<S: Storage> {
    command_handler: CommandHandler<S>,
    editor: DefaultEditor,
}

impl<S: Storage> Repl<S> {
    /// Create a new REPL
    pub fn new(storage: Arc<S>, source_factory: Arc<dyn SourceFactory>) -> RustylineResult<Self> {
        let command_handler = CommandHandler::new(storage, source_factory);
        let editor = DefaultEditor::new()?;

        Ok(Self {
            command_handler,
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

                    if line.trim() == "exit" || line.trim() == "quit" {
                        println!("Goodbye!");
                        break;
                    }

                    self.process_command(&line).await;
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
    async fn process_command(&self, line: &str) {
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
        let result = self.command_handler.execute(command, args).await;

        match result {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}: {}", "Error".bright_red(), err),
        }
    }
}
