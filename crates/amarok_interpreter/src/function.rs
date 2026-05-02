use amarok_syntax::{Spanned, Statement};

use crate::interpreter::Interpreter;
use crate::value::Value;
use amarok_syntax::Span;

pub(crate) type BuiltinFunction = fn(&mut Interpreter, Vec<Value>, Span) -> Value;

#[derive(Clone, Debug)]
pub(crate) enum Function {
    UserDefined {
        parameters: Vec<String>,
        body: Vec<Spanned<Statement>>,
    },
}
