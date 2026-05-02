//! Items registered in Amarok's `std::` namespace.
//!
//! Mirrors the spirit of Rust's `std`: things that may rely on host I/O. Only
//! registered when the interpreter is constructed via [`Interpreter::new`];
//! [`Interpreter::new_no_std`] omits this module so callers can opt out.

use amarok_syntax::Span;

use crate::function::BuiltinFunction;
use crate::interpreter::Interpreter;
use crate::value::{Value, format_value};

pub(crate) const NAMESPACE: &str = "std";

pub(crate) fn register(interpreter: &mut Interpreter) {
    interpreter.register_builtin(NAMESPACE, "print", std_print as BuiltinFunction);
}

fn std_print(interpreter: &mut Interpreter, arguments: Vec<Value>, _call_span: Span) -> Value {
    let mut pieces = Vec::new();
    for value in arguments {
        pieces.push(format_value(&value));
    }
    interpreter.push_output(pieces.join(" "));
    Value::Null
}
