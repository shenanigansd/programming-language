use amarok_lexer::{Token, TokenKind};

/// Most lexer tests care about the sequence of kinds, not exact offsets.
pub fn kinds(tokens: &[Token]) -> Vec<TokenKind> {
    tokens.iter().map(|token| token.kind.clone()).collect()
}
