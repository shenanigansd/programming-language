use crate::errors::{CommandError, ExitCode};

pub enum Command {
    Help,
    Version,
}

impl Command {
    pub fn parse(arguments: Vec<String>) -> Result<Self, CommandError> {
        if arguments.is_empty() {
            return Ok(Command::Help);
        }

        let command_name = &arguments[0];

        match command_name.as_str() {
            "help" => Ok(Command::Help),
            "version" => Ok(Command::Version),
            unknown => Err(CommandError::new(
                ExitCode::Usage,
                format!("Unknown command: {}", unknown),
            )),
        }
    }

    pub fn run(self) -> Result<(), CommandError> {
        match self {
            Command::Help => run_help(),
            Command::Version => run_version(),
        }
    }
}

fn run_help() -> Result<(), CommandError> {
    println!("Amarok");
    println!();
    println!("Available commands:");
    println!("  help        Display this help message");
    println!("  version     Display version information");
    println!();
    Ok(())
}

fn run_version() -> Result<(), CommandError> {
    println!("Amarok version 0.1.0");
    Ok(())
}
