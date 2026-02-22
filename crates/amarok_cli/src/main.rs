use std::env;
use std::fs;

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut arguments = env::args().skip(1);

    let Some(path) = arguments.next() else {
        return Err("Usage: amarok_cli <path-to-file.amarok>".to_string());
    };

    if arguments.next().is_some() {
        return Err("Too many arguments. Usage: amarok_cli <path-to-file.amarok>".to_string());
    }

    let source = fs::read_to_string(&path)
        .map_err(|error| format!("Amarok error: Failed to read file {path}: {error}"))?;

    let program = match amarok_parser::parse_program(&source) {
        Ok(program) => program,
        Err(error) => {
            return Err(format!(
                "Amarok error: {}",
                render_parse_error(&path, &source, &error)
            ));
        }
    };

    let mut interpreter = amarok_interpreter::Interpreter::new();

    let result = interpreter.run_program(&program);

    // Always print whatever was produced before the error (or program end).
    for line in interpreter.output_lines() {
        println!("{line}");
    }

    // Then report runtime error (with spans) if it happened.
    if let Err(error) = result {
        return Err(format!(
            "Amarok error: {}",
            render_runtime_error(&path, &source, &error)
        ));
    }

    Ok(())
}

// --- error rendering helpers ---

fn line_col_from_offset(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;

    for (index, ch) in source.char_indices() {
        if index >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

fn line_text_and_line_start(source: &str, line_number: usize) -> Option<(&str, usize)> {
    let mut current_line = 1usize;
    let mut line_start = 0usize;

    for (index, ch) in source.char_indices() {
        if current_line == line_number && ch == '\n' {
            return Some((&source[line_start..index], line_start));
        }
        if ch == '\n' {
            current_line += 1;
            line_start = index + 1;
        }
    }

    if current_line == line_number {
        Some((&source[line_start..], line_start))
    } else {
        None
    }
}

fn render_runtime_error(
    path: &str,
    source: &str,
    error: &amarok_interpreter::RuntimeError,
) -> String {
    if let Some(span) = error.span {
        let (line, col) = line_col_from_offset(source, span.start);
        let mut output = format!("{path}:{line}:{col}: {}\n", error.message);

        if let Some((line_text, line_start)) = line_text_and_line_start(source, line) {
            output.push_str(line_text);
            output.push('\n');

            let caret_start = span.start.saturating_sub(line_start);
            let caret_end = span.end.saturating_sub(line_start).max(caret_start + 1);

            output.push_str(&" ".repeat(caret_start));
            output.push_str(&"^".repeat(caret_end - caret_start));
            output.push('\n');
        }

        output
    } else {
        format!("{path}: {}\n", error.message)
    }
}

fn render_parse_error(path: &str, source: &str, error: &amarok_parser::ParseError) -> String {
    // Same structure as render_runtime_error, just reading error.message + error.span.
    if let Some(span) = error.span {
        let (line, col) = line_col_from_offset(source, span.start);
        let mut output = format!("{path}:{line}:{col}: {}\n", error.message);

        if let Some((line_text, line_start)) = line_text_and_line_start(source, line) {
            output.push_str(line_text);
            output.push('\n');

            let caret_start = span.start.saturating_sub(line_start);
            let caret_end = span.end.saturating_sub(line_start).max(caret_start + 1);

            output.push_str(&" ".repeat(caret_start));
            output.push_str(&"^".repeat(caret_end - caret_start));
            output.push('\n');
        }

        output
    } else {
        format!("{path}: {}\n", error.message)
    }
}
