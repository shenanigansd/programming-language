# Amarok

A tree-walking interpreter for a small imperative language, written in Rust.

```text
source text
   │
   ▼  amarok_parser::parse_program
amarok_syntax::Program  (AST + spans)
   │
   ▼  amarok_interpreter::Interpreter::run_program
output / diagnostics
```

## Workspace layout

| Crate | Role |
| --- | --- |
| `amarok_syntax` | AST nodes, spans, source map, and `Diagnostic` shared by every other crate. |
| `amarok_parser` | Pest grammar plus builders that turn parse trees into `Program` ASTs. |
| `amarok_interpreter` | Runtime: scopes, values, control flow, builtins, module loader. |
| `amarok_cli` | Command-line entry point that wires parsing, interpretation, and diagnostic rendering. |

Other directories:

- `examples/` — sample `.amarok` programs (`hello.amarok`, `uses_math.amarok`, …).
- `stdlib/std/` — the standard library, written in Amarok itself (`io.amarok`, `math.amarok`).

## Running

```sh
cargo run -p amarok_cli -- examples/hello.amarok --stdlib stdlib
cargo run -p amarok_cli -- examples/uses_math.amarok --stdlib stdlib
```

Pass `--no-std` to run without the bundled `std::` builtins.

## Development

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```
