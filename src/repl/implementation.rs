use std::sync::Arc;

use colored::Colorize;
use rustyline::{DefaultEditor, Result as RustylineResult};

use crate::command::{Command, CommandParser};
use crate::models::source::SourceFactory;
use crate::process::{CommandProcessor, CommandProcessorImpl};
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
        // Parse the command
        let command = match CommandParser::parse(line) {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("{}: {}", "Error".bright_red(), err);
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
            Err(err) => println!("{}: {}", "Error".bright_red(), err),
        }
    }
}
