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
