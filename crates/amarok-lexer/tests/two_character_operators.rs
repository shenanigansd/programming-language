mod common;

use amarok_lexer::{TokenKind, tokenize};
use common::kinds;

#[test]
fn two_character_comparison_operators_produce_their_token() {
    let cases = [
        (">=", TokenKind::GreaterEqual),
        ("<=", TokenKind::LessEqual),
        ("==", TokenKind::EqualEqual),
        ("!=", TokenKind::BangEqual),
    ];
    for (source, expected) in cases {
        let (actual, diagnostics) = tokenize(source);
        let want = vec![expected, TokenKind::EndOfFile];
        assert_eq!(
            kinds(&actual),
            want,
            "input {source:?} produced the wrong tokens"
        );
        assert!(diagnostics.is_empty());
    }
}

#[test]
fn one_character_comparison_operators_alone() {
    let cases = [
        (">", TokenKind::Greater),
        ("<", TokenKind::Less),
        ("=", TokenKind::Equal),
        ("!", TokenKind::Bang),
    ];
    for (source, expected) in cases {
        let (actual, diagnostics) = tokenize(source);
        let want = vec![expected, TokenKind::EndOfFile];
        assert_eq!(
            kinds(&actual),
            want,
            "input {source:?} produced the wrong tokens"
        );
        assert!(diagnostics.is_empty());
    }
}

#[test]
fn greater_followed_by_non_equal_does_not_become_greater_equal() {
    // The exact case you flagged: ">(" should be Greater followed by
    // LeftParenthesis, not "swallow the paren as part of some > = thing."
    let (tokens, diagnostics) = tokenize(">(");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Greater,
            TokenKind::LeftParenthesis,
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}
