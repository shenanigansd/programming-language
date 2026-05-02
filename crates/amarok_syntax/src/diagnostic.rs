use crate::Span;

/// A message attached to a (optional) source span.
///
/// Shared by parser and interpreter error types so that error rendering
/// (line/column lookup, caret pointing) can be implemented once.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<Span>,
}

impl Diagnostic {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    #[must_use]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
}
