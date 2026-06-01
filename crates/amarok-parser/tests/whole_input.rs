mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use common::token;

#[test]
fn trailing_tokens_after_a_complete_expression_are_an_error() {
    // 1 2  — the 2 has no business being there
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::EndOfFile),
    ];
    assert!(parse(tokens).is_err());
}

#[test]
fn an_expression_that_uses_every_token_succeeds() {
    // 1 + 2 — consumes everything, lands cleanly on EndOfFile
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::EndOfFile),
    ];
    assert!(parse(tokens).is_ok());
}
