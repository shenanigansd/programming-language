// Helpers are shared across the parser test files; not every file uses every
// helper, so allow dead code rather than warn per test binary.
#![allow(dead_code)]

use amarok_lexer::{SourcePosition, Token, TokenKind};
use amarok_syntax::{Expression, ExpressionKind};

/// Parser tests don't care about positions, only about kinds and the tree.
pub fn token(kind: TokenKind) -> Token {
    Token {
        kind,
        position: SourcePosition { character_index: 0 },
    }
}

/// Build an expression node at the zero position. The `token` helper emits every
/// token at index 0, so every parsed node lands at index 0 too — expected trees
/// match by building with this same position.
pub fn expression(kind: ExpressionKind) -> Expression {
    kind.at(SourcePosition { character_index: 0 })
}
