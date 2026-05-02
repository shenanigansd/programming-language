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
