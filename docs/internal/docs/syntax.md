---
icon: lucide/list-tree
---

# Syntax & AST

`amarok-syntax` defines the abstract syntax tree — the data the
[parser](parser.md) produces and the [interpreter](interpreter.md) consumes. It
holds no logic, only data types, and depends only on
[`amarok-diagnostics`](diagnostics.md) so that every node can remember where it
came from.

## Expressions

An expression node is its shape paired with a source position:

```rust
pub struct Expression {
    pub kind: ExpressionKind,
    pub position: SourcePosition,
}

pub enum ExpressionKind {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NilLiteral,
    Unary  { operator: UnaryOperator,  operand: Box<Expression> },
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    Variable(String),
}
```

Every node carries a `SourcePosition` so that a runtime failure can point a caret
at the right place. The convention is that the position is the node's most
error-relevant token: a literal or variable points at itself, while a unary or
binary operation points at its **operator** — so `1 + true` blames the `+`.

Nodes are built fluently with the `.at(position)` helper, which wraps an
`ExpressionKind` into an `Expression`:

```rust
ExpressionKind::NumberLiteral(5.0).at(position)
```

## Operators

Operators are small `Copy` enums, kept separate from the tokens that produce them
and the values they act on:

```rust
pub enum UnaryOperator { Negate, Not }

pub enum BinaryOperator {
    Add, Subtract, Multiply, Divide,
    Equal, NotEqual,
    Less, LessEqual, Greater, GreaterEqual,
}
```

The [parser](parser.md) maps token kinds onto these, and the
[interpreter](interpreter.md) matches on them to decide what to compute. Keeping
them in their own enums means neither stage has to know about the other's token
or value types.

## Statements

A program is a sequence of statements:

```rust
pub enum Statement {
    Let { name: String, initializer: Expression },
    Expression(Expression),
    Block(Vec<Statement>),
}
```

- **`Let`** binds `name` to the value of `initializer` in the current scope.
- **`Expression`** is an expression evaluated for its value — today, that value is
  what gets printed.
- **`Block`** is a brace-delimited sequence of statements that runs in its own
  nested [scope](interpreter.md).
