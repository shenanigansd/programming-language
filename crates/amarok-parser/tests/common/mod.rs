use amarok_lexer::{SourcePosition, Token, TokenKind};

/// Parser tests don't care about positions, only about kinds and the tree.
pub fn token(kind: TokenKind) -> Token {
    Token {
        kind,
        position: SourcePosition { character_index: 0 },
    }
}
