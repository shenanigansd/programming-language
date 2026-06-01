use amarok_diagnostics::Diagnostic;
use amarok_interpreter::{Environment, Value, evaluate};
use amarok_syntax::Expression;

/// Evaluate with no variables in scope — for the many tests that don't use any.
pub fn eval(expression: &Expression) -> Result<Value, Diagnostic> {
    evaluate(expression, &Environment::new())
}
