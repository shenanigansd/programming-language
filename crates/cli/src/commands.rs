use crate::errors::CommandError;
use driver::{CompilationOptions, compile_file};

pub enum Command {
    Help,
    Version,
    Compile { source_path: String },
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
            "compile" => {
                if arguments.len() < 2 {
                    return Err(CommandError::new(
                        "The compile command requires a source path.",
                    ));
                }
                let source_path = arguments[1].clone();
                Ok(Command::Compile { source_path })
            }
            unknown => Err(CommandError::new(format!("Unknown command: {}", unknown))),
        }
    }

    pub fn run(self) -> Result<(), CommandError> {
        match self {
            Command::Help => run_help(),
            Command::Version => run_version(),
            Command::Compile { source_path } => run_compile(source_path),
        }
    }
}

fn run_help() -> Result<(), CommandError> {
    println!("wolf — command line interface");
    println!();
    println!("Available commands:");
    println!("  help        Display this help message");
    println!("  version     Display version information");
    println!("  compile     Compile a source file");
    println!();
    Ok(())
}

fn run_version() -> Result<(), CommandError> {
    println!("wolf version 0.1.0");
    Ok(())
}

fn run_compile(source_path: String) -> Result<(), CommandError> {
    let options = CompilationOptions::simple();

    let output_path = compile_file(&source_path, &options)
        .map_err(|error| CommandError::new(format!("Compilation failed: {}", error)))?;

    println!("Executable written to {}", output_path.display());

    Ok(())
}
