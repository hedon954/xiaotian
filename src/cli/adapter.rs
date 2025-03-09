use crate::cli::commands::{
    AddCommands, Commands, DeleteCommands, FetchCommands, ListCommands, ShowCommands,
};
use crate::command::{
    AddCommand, Command as InternalCommand, DeleteCommand, FetchCommand, ListCommand, ShowCommand,
};

/// Adapter to convert from Clap commands to internal commands
pub struct CommandAdapter;

impl CommandAdapter {
    /// Convert a Clap command to an internal command
    pub fn adapt(command: Option<Commands>) -> Option<InternalCommand> {
        command.map(Self::map_command)
    }

    /// Map a Clap command to an internal command
    fn map_command(command: Commands) -> InternalCommand {
        match command {
            Commands::Add(add_cmd) => Self::map_add_command(add_cmd),
            Commands::List(list_cmd) => Self::map_list_command(list_cmd),
            Commands::Show(show_cmd) => Self::map_show_command(show_cmd),
            Commands::Delete(delete_cmd) => Self::map_delete_command(delete_cmd),
            Commands::Fetch(fetch_cmd) => Self::map_fetch_command(fetch_cmd),
            Commands::Clear => InternalCommand::Clear,
        }
    }

    /// Map a Clap add command to an internal add command
    fn map_add_command(command: AddCommands) -> InternalCommand {
        match command {
            AddCommands::Repository { owner, name } => {
                InternalCommand::Add(AddCommand::Repository { owner, name })
            }
            AddCommands::Subscription { owner, name } => {
                InternalCommand::Add(AddCommand::Subscription { owner, name })
            }
        }
    }

    /// Map a Clap list command to an internal list command
    fn map_list_command(command: ListCommands) -> InternalCommand {
        match command {
            ListCommands::Repositories => InternalCommand::List(ListCommand::Repositories),
            ListCommands::Subscriptions => InternalCommand::List(ListCommand::Subscriptions),
            ListCommands::Updates => InternalCommand::List(ListCommand::Updates),
        }
    }

    /// Map a Clap show command to an internal show command
    fn map_show_command(command: ShowCommands) -> InternalCommand {
        match command {
            ShowCommands::Repository { owner, name } => {
                InternalCommand::Show(ShowCommand::Repository { owner, name })
            }
            ShowCommands::Subscription { id } => {
                InternalCommand::Show(ShowCommand::Subscription { id })
            }
            ShowCommands::Updates { id, limit } => InternalCommand::Show(ShowCommand::Updates {
                subscription_id: id,
                limit,
            }),
        }
    }

    /// Map a Clap delete command to an internal delete command
    fn map_delete_command(command: DeleteCommands) -> InternalCommand {
        match command {
            DeleteCommands::Repository { id } => {
                InternalCommand::Delete(DeleteCommand::Repository { id })
            }
            DeleteCommands::Subscription { id } => {
                InternalCommand::Delete(DeleteCommand::Subscription { id })
            }
        }
    }

    /// Map a Clap fetch command to an internal fetch command
    fn map_fetch_command(command: FetchCommands) -> InternalCommand {
        match command {
            FetchCommands::Updates { id, days } => InternalCommand::Fetch(FetchCommand::Updates {
                subscription_id: id,
                days,
            }),
        }
    }
}
