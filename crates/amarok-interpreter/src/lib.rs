pub use amarok_diagnostics::Diagnostic;
use amarok_diagnostics::SourcePosition;
use amarok_syntax::{BinaryOperator, Expression, UnaryOperator};
use std::fmt;

/// A runtime value produced by evaluating an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(number) => write!(formatter, "{number}"),
            Value::String(text) => write!(formatter, "{text}"),
            Value::Boolean(boolean) => write!(formatter, "{boolean}"),
            Value::Nil => write!(formatter, "nil"),
        }
    }
}

/// Evaluate an expression to a value, or fail with a runtime diagnostic.
///
/// # Errors
///
/// Returns a diagnostic when evaluation encounters invalid operand types or
/// illegal operations such as division by zero.
pub fn evaluate(expression: &Expression) -> Result<Value, Diagnostic> {
    match expression {
        Expression::NumberLiteral(number) => Ok(Value::Number(*number)),
        Expression::StringLiteral(text) => Ok(Value::String(text.clone())),
        Expression::BooleanLiteral(boolean) => Ok(Value::Boolean(*boolean)),
        Expression::NilLiteral => Ok(Value::Nil),
        Expression::Unary { operator, operand } => {
            let operand_value = evaluate(operand)?;
            evaluate_unary(*operator, operand_value)
        }
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            // Evaluate both operands left-to-right, then apply the operator.
            let left_value = evaluate(left)?;
            let right_value = evaluate(right)?;
            evaluate_binary(*operator, left_value, right_value)
        }
    }
}

fn evaluate_unary(operator: UnaryOperator, operand: Value) -> Result<Value, Diagnostic> {
    match operator {
        UnaryOperator::Negate => match operand {
            Value::Number(number) => Ok(Value::Number(-number)),
            other => Err(Diagnostic::new(
                format!("cannot negate a {} value", type_name(&other)),
                SourcePosition { character_index: 0 },
            )),
        },
        UnaryOperator::Not => Ok(Value::Boolean(!is_truthy(&operand))),
    }
}

fn evaluate_binary(
    operator: BinaryOperator,
    left: Value,
    right: Value,
) -> Result<Value, Diagnostic> {
    match operator {
        BinaryOperator::Add => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            (left, right) => Err(runtime_error(format!(
                "cannot add a {} and a {}",
                type_name(&left),
                type_name(&right),
            ))),
        },
        BinaryOperator::Subtract => arithmetic(left, right, "subtract", |a, b| a - b),
        BinaryOperator::Multiply => arithmetic(left, right, "multiply", |a, b| a * b),
        BinaryOperator::Divide => match (left, right) {
            (Value::Number(_), Value::Number(0.0)) => {
                Err(runtime_error("division by zero"))
            }
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            (left, right) => Err(runtime_error(format!(
                "cannot divide a {} by a {}",
                type_name(&left),
                type_name(&right),
            ))),
        },
        BinaryOperator::Less => comparison(left, right, |a, b| a < b),
        BinaryOperator::LessEqual => comparison(left, right, |a, b| a <= b),
        BinaryOperator::Greater => comparison(left, right, |a, b| a > b),
        BinaryOperator::GreaterEqual => comparison(left, right, |a, b| a >= b),
        BinaryOperator::Equal => Ok(Value::Boolean(left == right)),
        BinaryOperator::NotEqual => Ok(Value::Boolean(left != right)),
    }
}

/// Apply a numeric arithmetic operation, erroring on non-number operands.
fn arithmetic(
    left: Value,
    right: Value,
    verb: &str,
    operation: fn(f64, f64) -> f64,
) -> Result<Value, Diagnostic> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(operation(a, b))),
        (left, right) => Err(runtime_error(format!(
            "cannot {} a {} and a {}",
            verb,
            type_name(&left),
            type_name(&right),
        ))),
    }
}

/// Apply a numeric comparison, erroring on non-number operands.
fn comparison(
    left: Value,
    right: Value,
    operation: fn(f64, f64) -> bool,
) -> Result<Value, Diagnostic> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(operation(a, b))),
        (left, right) => Err(runtime_error(format!(
            "cannot compare a {} and a {}",
            type_name(&left),
            type_name(&right),
        ))),
    }
}

fn runtime_error(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(message, SourcePosition { character_index: 0 })
}

/// The truthiness convention: `nil` and `false` are falsy; everything else,
/// including zero and the empty string, is truthy.
fn is_truthy(value: &Value) -> bool {
    !matches!(value, Value::Nil | Value::Boolean(false))
}

/// A short human-readable name for a value's type, for error messages.
fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Boolean(_) => "boolean",
        Value::Nil => "nil",
    }
}
