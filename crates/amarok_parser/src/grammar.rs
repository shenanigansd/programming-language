use amarok_syntax::{Diagnostic, Span};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "amarok.pest"]
pub(crate) struct AmarokGrammar;

pub(crate) fn pest_error_to_diagnostic(error: &pest::error::Error<Rule>) -> Diagnostic {
    use pest::error::InputLocation;

    let message = error.to_string();

    let span = match error.location {
        InputLocation::Span((start, end)) => Some(Span::new(start, end)),
        InputLocation::Pos(pos) => Some(Span::new(pos, pos + 1)),
    };

    Diagnostic { message, span }
}
