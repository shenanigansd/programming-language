use amarok_diagnostics::render;
use amarok_interpreter::evaluate;
use amarok_lexer::tokenize;
use amarok_parser::parse;

/// Run one line of Amarok source and return the text to display: either the
/// resulting value, or a description of whatever error stopped us.
#[must_use]
pub fn run_line(source: &str) -> String {
    if source.trim().is_empty() {
        return String::new();
    }

    let (tokens, lex_diagnostics) = tokenize(source);
    if !lex_diagnostics.is_empty() {
        return lex_diagnostics
            .iter()
            .map(|diagnostic| render(source, diagnostic))
            .collect::<Vec<_>>()
            .join("\n");
    }

    let expression = match parse(tokens) {
        Ok(expression) => expression,
        Err(diagnostic) => return format!("parse error: {}", diagnostic.message),
    };

    match evaluate(&expression) {
        Ok(value) => value.to_string(),
        Err(diagnostic) => format!("runtime error: {}", diagnostic.message),
    }
}
