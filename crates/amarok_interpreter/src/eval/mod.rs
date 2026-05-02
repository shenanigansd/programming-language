//! Free-function execution logic that operates on `&mut Interpreter`.
//!
//! Split out from `interpreter.rs` so that file can stay focused on the
//! public API. The submodules here hold the recursive AST walk:
//! - `statements` — statement-level execution and control flow
//! - `expressions` — expression evaluation and function dispatch
//! - `modules` — `use` statement orchestration on top of `ModuleLoader`

pub(crate) mod expressions;
pub(crate) mod modules;
pub(crate) mod statements;
