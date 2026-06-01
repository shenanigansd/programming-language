use amarok_diagnostics::render;
use amarok_interpreter::{Environment, execute_statement};
use amarok_lexer::tokenize;
use amarok_parser::parse_program;

/// Run a whole source program and return its output as text — the value of each
/// expression statement (temporary, until we have a `print` function), or the
/// first error encountered, rendered with a caret where we have a position.
#[allow(clippy::must_use_candidate)]
pub fn run_source(source: &str) -> String {
    let (tokens, lex_diagnostics) = tokenize(source);
    if !lex_diagnostics.is_empty() {
        return lex_diagnostics
            .iter()
            .map(|diagnostic| render(source, diagnostic))
            .collect::<Vec<_>>()
            .join("\n");
    }

    let statements = match parse_program(tokens) {
        Ok(statements) => statements,
        Err(diagnostic) => return render(source, &diagnostic),
    };

    let mut environment = Environment::new();
    let mut output = Vec::new();
    for statement in &statements {
        match execute_statement(statement, &mut environment) {
            Ok(Some(value)) => output.push(value.to_string()),
            Ok(None) => {}
            Err(diagnostic) => {
                output.push(render(source, &diagnostic));
                break; // stop at the first runtime error
            }
        }
    }
    output.join("\n")
}
