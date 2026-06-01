---
icon: lucide/scan-text
---

# Lexer

`amarok-lexer` turns source text into a flat sequence of tokens. Its public
surface is one function:

```rust
pub fn tokenize(source: &str) -> (Vec<Token>, Vec<Diagnostic>)
```

It returns the tokens it produced *and* any [diagnostics](diagnostics.md) it
collected along the way — scanning never aborts (see
[Error recovery](#error-recovery)).

## Tokens

A `Token` is a kind plus the position where it began:

```rust
pub struct Token {
    pub kind: TokenKind,
    pub position: SourcePosition,
}
```

`TokenKind` is an enum covering punctuation (`(` `)` `{` `}` `,` `.` `;`),
operators, literals that carry a payload (`NumberLiteral(f64)`,
`StringLiteral(String)`, `Identifier(String)`), keywords, and an `EndOfFile`
sentinel that always terminates the stream.

## Scanning

The lexer holds the source as a `Vec<char>` and a cursor. It makes a single
forward pass; each step reads one character and decides what to do using a few
helpers:

- `advance` — consume and return the current character;
- `peek` / `peek_next` — look at the current / next character without consuming;
- `match_next` — consume the next character only if it equals an expected one.

`match_next` is what lets the lexer prefer the longer of two operators — a
maximal-munch rule. On seeing `!` it consumes a following `=` to make
`BangEqual`, otherwise it emits `Bang`; the same pattern produces `==`, `<=`, and
`>=`.

A `/` is similar but special: a second `/` turns the rest of the line into a
comment, which is skipped entirely rather than tokenized. Spaces, tabs, carriage
returns, and newlines are skipped too.

### Literals

- **Numbers** are a run of digits, optionally followed by a `.` and more digits,
  parsed into an `f64`. A trailing `.` with no digit after it is not taken as part
  of the number.
- **Strings** are opened by `"` and run until the closing `"`.

### Identifiers and keywords

A character that can start an identifier — a letter or `_` — begins a run of
letters, digits, and underscores. That text is then looked up in a keyword table:
matches become keyword tokens, and everything else becomes an `Identifier`
carrying its name. The keywords the rest of the pipeline acts on today are `let`,
`true`, `false`, `nil`, and `not`.

## Error recovery

The lexer is built to keep going so that one mistake doesn't hide the rest of the
file. Two situations produce a [`Diagnostic`](diagnostics.md) without stopping the
scan:

- an **unterminated string** — end of input before a closing `"` — records the
  error and still emits a best-effort token with whatever was read;
- an **unexpected character** that can't begin any token records the error and
  skips that one character.

Because scanning continues, a single run can report several lexical errors. The
[driver](architecture.md) treats the presence of *any* lexer diagnostic as fatal
and renders them all before parsing begins.
