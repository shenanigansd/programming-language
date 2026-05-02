//! Command-line entry point for Amarok.
//!
//! Parses arguments, reads the source file, registers it with a
//! [`SourceMap`], invokes [`amarok_parser`] then [`amarok_interpreter`],
//! and renders any [`Diagnostic`] with line and column information.

use amarok_syntax::{Diagnostic, FileId, SourceMap};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const USAGE: &str = "Usage: amarok_cli [--no-std] [--stdlib <dir>] <path-to-file.amarok>";

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut arguments = env::args().skip(1);

    let mut no_std = false;
    let mut stdlib_override: Option<PathBuf> = None;
    let mut path: Option<String> = None;

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--no-std" => no_std = true,
            "--stdlib" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| format!("--stdlib requires a directory.\n{USAGE}"))?;
                stdlib_override = Some(PathBuf::from(value));
            }
            "--" => {
                path = arguments.next();
                break;
            }
            other if other.starts_with("--") => {
                return Err(format!("Unknown option: {other}\n{USAGE}"));
            }
            _ => {
                path = Some(argument);
                break;
            }
        }
    }

    let Some(path) = path else {
        return Err(format!("Missing source file.\n{USAGE}"));
    };

    if arguments.next().is_some() {
        return Err(format!("Too many arguments.\n{USAGE}"));
    }

    let entry_path = PathBuf::from(&path);
    let source = fs::read_to_string(&entry_path)
        .map_err(|error| format!("Amarok error: Failed to read file {path}: {error}"))?;

    // Build the SourceMap and register the entry file before parsing so its
    // spans carry the right FileId for later diagnostics.
    let mut source_map = SourceMap::new();
    let entry_canonical = entry_path
        .canonicalize()
        .unwrap_or_else(|_| entry_path.clone());
    let entry_file_id = source_map.add_file(entry_canonical.clone(), source.clone());

    let program = match amarok_parser::parse_program_with_file_id(&source, entry_file_id) {
        Ok(program) => program,
        Err(error) => {
            return Err(format!(
                "Amarok error: {}",
                render_diagnostic(&source_map, &entry_path, &source, &error)
            ));
        }
    };

    let mut interpreter = if no_std {
        amarok_interpreter::Interpreter::new_no_std()
    } else {
        amarok_interpreter::Interpreter::new()
    };

    // Module search roots: project root (entry file's directory), then stdlib.
    let project_root = entry_canonical
        .parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    interpreter.add_module_root(project_root);

    if let Some(dir) = stdlib_override
        .or_else(|| env::var_os("AMAROK_STDLIB").map(PathBuf::from))
        .or_else(default_stdlib_dir)
    {
        interpreter.add_module_root(dir);
    }

    interpreter.set_source_map(source_map);

    let result = interpreter.run_program(&program);

    // Always print whatever was produced before the error (or program end).
    for line in interpreter.output_lines() {
        println!("{line}");
    }

    if let Err(error) = result {
        let source_map = interpreter.source_map();
        return Err(format!(
            "Amarok error: {}",
            render_diagnostic(source_map, &entry_path, &source, &error)
        ));
    }

    Ok(())
}

fn default_stdlib_dir() -> Option<PathBuf> {
    // Dev convenience: <repo>/stdlib relative to this crate's manifest.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let candidate = Path::new(manifest_dir).join("..").join("..").join("stdlib");
    if candidate.is_dir() {
        Some(candidate)
    } else {
        None
    }
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

fn render_diagnostic(
    source_map: &SourceMap,
    entry_path: &Path,
    entry_source: &str,
    diagnostic: &Diagnostic,
) -> String {
    let Some(span) = diagnostic.span else {
        return format!("{}: {}\n", entry_path.display(), diagnostic.message);
    };

    // Resolve the file the span belongs to: prefer the source map; fall back
    // to the entry file (covers spans without a registered FileId).
    let (path_display, source_text): (String, &str) =
        if span.file_id != FileId::DUMMY && source_map.get(span.file_id).is_some() {
            let file = source_map.get(span.file_id).unwrap();
            (file.path.display().to_string(), file.source.as_str())
        } else {
            (entry_path.display().to_string(), entry_source)
        };

    let (line, col) = line_col_from_offset(source_text, span.start);
    let mut output = format!("{path_display}:{line}:{col}: {}\n", diagnostic.message);

    if let Some((line_text, line_start)) = line_text_and_line_start(source_text, line) {
        output.push_str(line_text);
        output.push('\n');

        let caret_start = span.start.saturating_sub(line_start);
        let caret_end = span.end.saturating_sub(line_start).max(caret_start + 1);

        output.push_str(&" ".repeat(caret_start));
        output.push_str(&"^".repeat(caret_end - caret_start));
        output.push('\n');
    }

    output
}
