use std::env;
use std::fs;
use std::process;

use amarok_cli::run_source;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        eprintln!("usage: amarok <source-file>");
        process::exit(64); // 64 = command-line usage error (BSD sysexits)
    }

    let path = &arguments[1];
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("could not read '{path}': {error}");
            process::exit(66); // 66 = cannot open input
        }
    };

    let output = run_source(&source);
    if !output.is_empty() {
        println!("{output}");
    }
}
