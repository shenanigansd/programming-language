mod common;

use amarok_lexer::{TokenKind, tokenize};
use common::kinds;

#[test]
fn each_single_character_punctuation_produces_its_token() {
    let cases = [
        ("(", TokenKind::LeftParenthesis),
        (")", TokenKind::RightParenthesis),
        ("{", TokenKind::LeftBrace),
        ("}", TokenKind::RightBrace),
        (",", TokenKind::Comma),
        (".", TokenKind::Dot),
        ("-", TokenKind::Minus),
        ("+", TokenKind::Plus),
        (";", TokenKind::Semicolon),
        ("*", TokenKind::Star),
    ];

    for (source, expected) in cases {
        let (tokens, diagnostics) = tokenize(source);
        assert_eq!(
            kinds(&tokens),
            vec![expected, TokenKind::EndOfFile],
            "input {source:?}"
        );
        assert!(
            diagnostics.is_empty(),
            "input {source:?} produced unexpected diagnostics"
        );
    }
}
