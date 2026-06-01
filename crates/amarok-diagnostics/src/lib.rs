/// A location within a source file, identified by a zero-based index into the
/// sequence of characters. Line and column numbers can be derived from this
/// index plus the original source text when we render errors for humans; the
/// stored position stays minimal for now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub character_index: usize,
}

/// A problem discovered while processing source code — for example, an
/// unexpected character during lexing, or later a syntax error during parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    /// A human-readable description of the problem.
    pub message: String,
    /// Where in the source the problem was found.
    pub position: SourcePosition,
}

impl Diagnostic {
    /// Create a diagnostic with the given message at the given position. The
    /// message accepts anything convertible into a `String`, so callers can
    /// pass either a string literal or an owned `String`.
    pub fn new(message: impl Into<String>, position: SourcePosition) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

/// Render a diagnostic against its source as a two-line display: the offending
/// line of source, then a caret pointing at the column, followed by the message.
///
/// Assumes one character is one column wide — tabs and wide characters aren't
/// handled yet, which is fine for now.
#[allow(clippy::must_use_candidate)]
pub fn render(source: &str, diagnostic: &Diagnostic) -> String {
    let characters: Vec<char> = source.chars().collect();
    let target = diagnostic.position.character_index;

    // Walk up to the target, tracking which line we're on and where it started.
    let mut line_start = 0;
    for (index, character) in characters.iter().enumerate().take(target.min(characters.len())) {
        if *character == '\n' {
            line_start = index + 1;
        }
    }

    // Column within the line, and the text of that line up to its newline.
    let column = target.saturating_sub(line_start);
    let line_text: String = characters[line_start..]
        .iter()
        .take_while(|&&character| character != '\n')
        .collect();

    let indent = " ".repeat(column);
    format!(
        "{line_text}\n{indent}^ {message}",
        message = diagnostic.message
    )
}
