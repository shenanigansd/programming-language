use std::io::{self, Write};

use amarok_cli::run_line;

fn main() {
    println!("Amarok. Type an expression, or press Ctrl-D to exit.");
    let stdin = io::stdin();
    loop {
        print!("> ");
        // Flush so the prompt shows before we block waiting for input.
        io::stdout().flush().expect("stdout should be flushable");

        let mut line = String::new();
        let bytes_read = stdin
            .read_line(&mut line)
            .expect("stdin should be readable");
        if bytes_read == 0 {
            // Ctrl-D / end of input.
            println!();
            break;
        }

        let output = run_line(&line);
        if !output.is_empty() {
            println!("{output}");
        }
    }
}
