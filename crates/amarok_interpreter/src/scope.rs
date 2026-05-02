use std::collections::HashMap;

use amarok_syntax::{Diagnostic, Span};

use crate::value::Value;

/// Lexically nested variable scopes.
///
/// The outermost scope is at index 0; the innermost is `last`.
/// At least one scope is always present.
pub(crate) struct ScopeStack {
    scopes: Vec<HashMap<String, Value>>,
}

impl ScopeStack {
    pub(crate) fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub(crate) fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub(crate) fn pop(&mut self) {
        self.scopes.pop();
        if self.scopes.is_empty() {
            self.scopes.push(HashMap::new());
        }
    }

    /// Insert into the current (innermost) scope.
    pub(crate) fn set_innermost(&mut self, name: &str, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), value);
        }
    }

    /// Look up a name from innermost to outermost.
    pub(crate) fn get(&self, name: &str, span: Span) -> Result<Value, Diagnostic> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(Diagnostic::new(format!("Undefined variable: {name}")).with_span(span))
    }
}
