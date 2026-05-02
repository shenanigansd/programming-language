//! Items registered in Amarok's `core::` namespace.
//!
//! Reserved namespace for host-independent primitives, mirroring the spirit
//! of Rust's `core`. Currently empty — registration creates the namespace so
//! lookups like `core::foo(...)` can produce a clear "unknown path" error
//! rather than a "no such namespace" one once items start landing here.

use crate::interpreter::Interpreter;

pub(crate) const NAMESPACE: &str = "core";

pub(crate) fn register(interpreter: &mut Interpreter) {
    interpreter.ensure_namespace(NAMESPACE);
    // No items yet — reserved for future additions.
}
