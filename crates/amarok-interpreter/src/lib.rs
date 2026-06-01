pub use amarok_diagnostics::Diagnostic;
use amarok_diagnostics::SourcePosition;
use amarok_syntax::{BinaryOperator, Expression, ExpressionKind, Statement, UnaryOperator};
use std::collections::HashMap;
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

/// The set of variable bindings in scope during evaluation.
#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind (or rebind) a name to a value.
    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        self.values.insert(name.into(), value);
    }

    /// Look up a name, returning the bound value if there is one.
    #[allow(clippy::must_use_candidate)]
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}

/// Evaluate an expression to a value, or fail with a runtime diagnostic.
///
/// # Errors
///
/// Returns a diagnostic when evaluation encounters invalid operand types or
/// illegal operations such as division by zero.
pub fn evaluate(expression: &Expression, environment: &Environment) -> Result<Value, Diagnostic> {
    match &expression.kind {
        ExpressionKind::NumberLiteral(number) => Ok(Value::Number(*number)),
        ExpressionKind::StringLiteral(text) => Ok(Value::String(text.clone())),
        ExpressionKind::BooleanLiteral(boolean) => Ok(Value::Boolean(*boolean)),
        ExpressionKind::NilLiteral => Ok(Value::Nil),
        ExpressionKind::Variable(name) => match environment.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(runtime_error(
                format!("undefined variable '{name}'"),
                expression.position,
            )),
        },
        ExpressionKind::Unary { operator, operand } => {
            let operand_value = evaluate(operand, environment)?;
            evaluate_unary(*operator, operand_value, expression.position)
        }
        ExpressionKind::Binary {
            left,
            operator,
            right,
        } => {
            let left_value = evaluate(left, environment)?;
            let right_value = evaluate(right, environment)?;
            evaluate_binary(left_value, *operator, right_value, expression.position)
        }
    }
}

fn evaluate_unary(
    operator: UnaryOperator,
    operand: Value,
    position: SourcePosition,
) -> Result<Value, Diagnostic> {
    match operator {
        UnaryOperator::Negate => match operand {
            Value::Number(number) => Ok(Value::Number(-number)),
            other => Err(runtime_error(
                format!("cannot negate a {} value", type_name(&other)),
                position,
            )),
        },
        UnaryOperator::Not => Ok(Value::Boolean(!is_truthy(&operand))),
    }
}

fn evaluate_binary(
    left: Value,
    operator: BinaryOperator,
    right: Value,
    position: SourcePosition,
) -> Result<Value, Diagnostic> {
    match operator {
        BinaryOperator::Add => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            (left, right) => Err(runtime_error(
                format!(
                    "cannot add a {} and a {}",
                    type_name(&left),
                    type_name(&right),
                ),
                position,
            )),
        },
        BinaryOperator::Subtract => arithmetic(left, right, "subtract", position, |a, b| a - b),
        BinaryOperator::Multiply => arithmetic(left, right, "multiply", position, |a, b| a * b),
        BinaryOperator::Divide => match (left, right) {
            (Value::Number(_), Value::Number(0.0)) => {
                Err(runtime_error("division by zero", position))
            }
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            (left, right) => Err(runtime_error(
                format!(
                    "cannot divide a {} by a {}",
                    type_name(&left),
                    type_name(&right),
                ),
                position,
            )),
        },
        BinaryOperator::Less => comparison(left, right, position, |a, b| a < b),
        BinaryOperator::LessEqual => comparison(left, right, position, |a, b| a <= b),
        BinaryOperator::Greater => comparison(left, right, position, |a, b| a > b),
        BinaryOperator::GreaterEqual => comparison(left, right, position, |a, b| a >= b),
        BinaryOperator::Equal => Ok(Value::Boolean(left == right)),
        BinaryOperator::NotEqual => Ok(Value::Boolean(left != right)),
    }
}

/// Apply a numeric arithmetic operation, erroring on non-number operands.
fn arithmetic(
    left: Value,
    right: Value,
    verb: &str,
    position: SourcePosition,
    operation: fn(f64, f64) -> f64,
) -> Result<Value, Diagnostic> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(operation(a, b))),
        (left, right) => Err(runtime_error(
            format!(
                "cannot {} a {} and a {}",
                verb,
                type_name(&left),
                type_name(&right),
            ),
            position,
        )),
    }
}

/// Apply a numeric comparison, erroring on non-number operands.
fn comparison(
    left: Value,
    right: Value,
    position: SourcePosition,
    operation: fn(f64, f64) -> bool,
) -> Result<Value, Diagnostic> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(operation(a, b))),
        (left, right) => Err(runtime_error(
            format!(
                "cannot compare a {} and a {}",
                type_name(&left),
                type_name(&right),
            ),
            position,
        )),
    }
}

fn runtime_error(message: impl Into<String>, position: SourcePosition) -> Diagnostic {
    Diagnostic::new(message, position)
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

/// Execute one statement against the environment. Returns the statement's
/// value if it produces one (an expression statement does; a declaration
/// doesn't), so a caller can decide what to do with it.
///
/// # Errors
///
/// Returns a [`Diagnostic`] when evaluating the statement fails at runtime,
/// such as applying an operation to values of incompatible types.
pub fn execute_statement(
    statement: &Statement,
    environment: &mut Environment,
) -> Result<Option<Value>, Diagnostic> {
    match statement {
        Statement::Let { name, initializer } => {
            let value = evaluate(initializer, environment)?;
            environment.define(name.clone(), value);
            Ok(None)
        }
        Statement::Expression(expression) => {
            let value = evaluate(expression, environment)?;
            Ok(Some(value))
        }
    }
}
