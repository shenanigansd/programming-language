// Helpers are shared across the interpreter test files; not every file uses
// every helper, so allow dead code rather than warn per test binary.
#![allow(dead_code)]

use amarok_diagnostics::{Diagnostic, SourcePosition};
use amarok_interpreter::{Environment, Value, evaluate};
use amarok_syntax::{Expression, ExpressionKind};

/// Evaluate with no variables in scope — for the many tests that don't use any.
pub fn eval(expression: &Expression) -> Result<Value, Diagnostic> {
    evaluate(expression, &Environment::new())
}

/// Build an expression node at the zero position. Evaluation only reads the
/// position on error paths (to point a caret), and these tests don't assert on
/// it, so a zero position is fine everywhere.
pub fn expression(kind: ExpressionKind) -> Expression {
    kind.at(SourcePosition { character_index: 0 })
}
