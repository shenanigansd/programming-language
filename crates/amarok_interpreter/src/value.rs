use amarok_syntax::{BinaryOperator, Diagnostic, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    String(String),
    Null,
}

pub(crate) fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Integer(v) => *v != 0,
        Value::String(s) => !s.is_empty(),
    }
}

pub(crate) fn format_value(value: &Value) -> String {
    match value {
        Value::Integer(v) => v.to_string(),
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
    }
}

pub(crate) fn evaluate_binary(
    operator: BinaryOperator,
    left: Value,
    right: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    match (operator, left, right) {
        (BinaryOperator::Add, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
        (BinaryOperator::Subtract, Value::Integer(a), Value::Integer(b)) => {
            Ok(Value::Integer(a - b))
        }
        (BinaryOperator::Multiply, Value::Integer(a), Value::Integer(b)) => {
            Ok(Value::Integer(a * b))
        }
        (BinaryOperator::Divide, Value::Integer(a), Value::Integer(b)) => {
            if b == 0 {
                Err(Diagnostic::new("Division by zero.").with_span(span))
            } else {
                Ok(Value::Integer(a / b))
            }
        }

        // Convenience: string concatenation for "+"
        (BinaryOperator::Add, Value::String(a), Value::String(b)) => {
            Ok(Value::String(format!("{a}{b}")))
        }

        (op, a, b) => {
            Err(Diagnostic::new(format!("Unsupported operation: {a:?} {op} {b:?}")).with_span(span))
        }
    }
}
