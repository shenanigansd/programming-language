mod common;

use amarok_lexer::{TokenKind, tokenize};
use common::kinds;

#[test]
fn spaces_between_tokens_are_skipped() {
    let (tokens, diagnostics) = tokenize("  (  )  ");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::LeftParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn tabs_and_newlines_and_carriage_returns_are_also_skipped() {
    let (tokens, diagnostics) = tokenize("(\n\t\r)");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::LeftParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn input_of_only_whitespace_produces_only_end_of_file() {
    let (tokens, diagnostics) = tokenize("   \n\t  ");
    assert_eq!(kinds(&tokens), vec![TokenKind::EndOfFile]);
    assert!(diagnostics.is_empty());
}
