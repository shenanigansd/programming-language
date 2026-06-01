mod common;

use amarok_lexer::{TokenKind, tokenize};
use common::kinds;

#[test]
fn single_slash_is_division() {
    let (tokens, diagnostics) = tokenize("/");
    assert_eq!(kinds(&tokens), vec![TokenKind::Slash, TokenKind::EndOfFile]);
    assert!(diagnostics.is_empty());
}

#[test]
fn line_comment_until_end_of_input_produces_no_tokens() {
    let (tokens, diagnostics) = tokenize("// this is a comment");
    assert_eq!(kinds(&tokens), vec![TokenKind::EndOfFile]);
    assert!(diagnostics.is_empty());
}

#[test]
fn line_comment_ends_at_newline_and_following_tokens_still_appear() {
    let (tokens, diagnostics) = tokenize("// comment\n+");
    assert_eq!(kinds(&tokens), vec![TokenKind::Plus, TokenKind::EndOfFile]);
    assert!(diagnostics.is_empty());
}

#[test]
fn empty_comment_with_nothing_after_the_double_slash() {
    let (tokens, diagnostics) = tokenize("//");
    assert_eq!(kinds(&tokens), vec![TokenKind::EndOfFile]);
    assert!(diagnostics.is_empty());
}

#[test]
fn division_between_other_tokens() {
    let (tokens, diagnostics) = tokenize("(/)");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::LeftParenthesis,
            TokenKind::Slash,
            TokenKind::RightParenthesis,
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}
