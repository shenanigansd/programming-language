mod commands;
mod errors;

use crate::commands::Command;
use crate::errors::ExitCode;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let result = Command::parse(args).and_then(|c| c.run());

    if let Err(err) = result {
        eprintln!("{}", err.message);
        std::process::exit(match err.code {
            ExitCode::Usage => 64,
            ExitCode::Compile => 65,
            ExitCode::Runtime => 70,
        });
    }
}
