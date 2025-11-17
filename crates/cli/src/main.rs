mod commands;
mod errors;

use crate::commands::Command;

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    match Command::parse(arguments) {
        Ok(command) => {
            if let Err(error) = command.run() {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
