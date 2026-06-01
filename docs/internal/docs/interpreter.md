---
icon: lucide/play
---

# Interpreter

`amarok-interpreter` is the back end: it walks the [AST](syntax.md) and produces
[values](#values). It is a tree-walking interpreter — there is no bytecode or
separate compilation step. Two functions drive it:

- `evaluate(&Expression, &SharedEnvironment) -> Result<Value, Diagnostic>` —
  reduce an expression to a value.
- `execute_statement(&Statement, &SharedEnvironment) -> Result<Option<Value>, Diagnostic>`
  — run a statement, returning `Some(value)` for an expression statement and
  `None` for a `let` or a block.

## Values

A runtime value is one of four kinds:

```rust
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
```

`Value` implements `Display` so the driver can print results: numbers, strings,
and booleans print as themselves, and `Nil` prints as `nil`.

### Truthiness

Anywhere a value is treated as a condition — `not`, for example — one rule
applies: `nil` and `false` are falsy, and **everything else is truthy**,
including `0` and the empty string.

## Evaluating expressions

`evaluate` matches on the expression's kind:

- **Literals** become the corresponding `Value` directly.
- A **variable** is looked up in the environment; an unknown name is a runtime
  error (`undefined variable '…'`).
- A **unary** operation evaluates its operand, then applies `Negate` (numbers
  only) or `Not` (any value, via truthiness).
- A **binary** operation evaluates both operands — left, then right — and combines
  them.

Binary operators are type-checked as they run:

| Operator(s) | Operands | Notes |
|-------------|----------|-------|
| `+` | number + number, or string + string | the one overloaded operator: adds numbers or concatenates strings |
| `-` `*` `/` | number, number | `/` reports `division by zero` when the divisor is `0` |
| `<` `<=` `>` `>=` | number, number | yields a boolean |
| `==` `!=` | any, any | structural equality across `Value`s |

Any disallowed combination — `1 + true`, negating a string, comparing
non-numbers — produces a [`Diagnostic`](diagnostics.md) at the operator's
position, which is exactly the position the [parser](parser.md) recorded on the
node.

## Environments and scope

Variables live in `Environment`s. Each holds its own names and an optional link to
the scope that encloses it:

```rust
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<SharedEnvironment>,
}
```

Environments are shared through `SharedEnvironment = Rc<RefCell<Environment>>`, so
a child scope and its parent can hold the same environment by reference. A lookup
(`get`) checks the current scope first, then walks outward through `enclosing`
until the name is found or the chain ends:

```mermaid
flowchart LR
    block[block scope] -->|enclosing| global[global scope]
    global -->|enclosing| none[none]
```

A program starts in a single global environment. Executing a `Block` creates a
fresh child scope with `Environment::new_child`, runs the block's statements in
it, and discards it afterwards — so names declared inside a block don't leak out,
while the block can still read names from the scopes around it.

## Executing statements

`execute_statement` ties expressions and scopes together:

- **`Let`** evaluates its initializer and `define`s the name in the current scope.
- **`Expression`** evaluates the expression and hands back its value — this is what
  the driver prints.
- **`Block`** runs its statements in a new child scope, as above.
