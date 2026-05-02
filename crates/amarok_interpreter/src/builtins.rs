use amarok_syntax::Span;

use crate::function::BuiltinFunction;
use crate::interpreter::Interpreter;
use crate::value::{Value, format_value};

pub(crate) fn register_builtins(interpreter: &mut Interpreter) {
    interpreter
        .builtins_mut()
        .insert("print".to_string(), builtin_print as BuiltinFunction);
}

fn builtin_print(interpreter: &mut Interpreter, arguments: Vec<Value>, _call_span: Span) -> Value {
    let mut pieces = Vec::new();
    for value in arguments {
        pieces.push(format_value(&value));
    }
    interpreter.push_output(pieces.join(" "));
    Value::Null
}
