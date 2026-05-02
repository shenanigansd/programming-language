//! Tree-walking interpreter for Amarok.
//!
//! Public surface:
//! - [`Interpreter`] — owns scopes, functions, builtins, and captured output
//! - [`Value`] — runtime values (Integer, String, Null)
//!
//! Errors use [`amarok_syntax::Diagnostic`] directly (re-exported here for
//! convenience), shared with the parser so a single rendering routine
//! handles both phases.
//!
//! Internal modules:
//! - `interpreter` — the [`Interpreter`] struct and execution logic
//! - `value` — [`Value`] enum and formatting/truthiness helpers
//! - `control_flow` — control-flow signal returned by statement execution
//! - `function` — user-defined function representation and builtin signature
//! - `scope` — lexical scope stack
//! - `builtins` — registration and implementation of builtin functions

mod builtins;
mod control_flow;
mod function;
mod interpreter;
mod scope;
mod value;

pub use amarok_syntax::Diagnostic;
pub use interpreter::Interpreter;
pub use value::Value;
