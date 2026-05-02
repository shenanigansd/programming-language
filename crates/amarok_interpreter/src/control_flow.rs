use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ControlFlow {
    Continue,
    Return(Value),
}
