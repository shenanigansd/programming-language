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
//! - `interpreter` — the [`Interpreter`] struct and its public API
//! - `eval` — recursive AST walk (statements + expressions)
//! - `value` — [`Value`] enum and formatting/truthiness helpers
//! - `control_flow` — control-flow signal returned by statement execution
//! - `function` — user-defined function representation and builtin signature
//! - `scope` — lexical scope stack
//! - `module_loader` — `use` statement resolution and module caching
//! - `std_lib` — registration of items in the `std::` namespace

mod control_flow;
mod eval;
mod function;
mod interpreter;
mod module_loader;
mod scope;
mod std_lib;
mod value;

pub use amarok_syntax::Diagnostic;
pub use interpreter::Interpreter;
pub use value::Value;
