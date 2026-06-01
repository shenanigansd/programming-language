---
icon: lucide/bug
---

# Diagnostics

`amarok-diagnostics` is the foundation the other crates are built on. It defines
two things — *where* a problem is and *what* the problem is — plus how to show it
to a human. It has no dependencies of its own.

## Source positions

A location in the source is a single zero-based index into the program's
characters:

```rust
pub struct SourcePosition {
    pub character_index: usize,
}
```

This is deliberately minimal. Line and column numbers are not stored; they are
derived from the index and the original source only when an error is rendered.
Keeping a position to one number means every token and every AST node can carry
one cheaply.

## Diagnostics

A `Diagnostic` pairs a human-readable message with the position it refers to:

```rust
pub struct Diagnostic {
    pub message: String,
    pub position: SourcePosition,
}
```

`Diagnostic::new` accepts anything that converts into a `String`, so callers can
pass either a string literal or an owned, formatted message:

```rust
Diagnostic::new("unterminated string literal", start_position);
Diagnostic::new(format!("undefined variable '{name}'"), position);
```

Each stage produces `Diagnostic`s in its own way — the [lexer](lexer.md) collects
them while scanning, the [parser](parser.md) returns one when it meets an
unexpected token, and the [interpreter](interpreter.md) returns one on a type
mismatch or division by zero — but they are all the same type, so the driver can
render any of them the same way.

## Rendering

`render(source, diagnostic) -> String` turns a diagnostic into a two-line
message: the offending line of source, then a caret under the column, followed by
the message. Conceptually it:

1. walks the source up to the target index, remembering where the current line
   started;
2. computes the column as the distance from that line start;
3. slices out the text of that line up to the next newline;
4. emits the line, then `column` spaces, a `^`, and the message.

So a diagnostic at the `=` in `let = 5;` renders as:

```text
let = 5;
    ^ expected a variable name after 'let', found Equal
```

Rendering assumes one character occupies one column, so for ASCII source the
caret lands exactly under the offending character.
