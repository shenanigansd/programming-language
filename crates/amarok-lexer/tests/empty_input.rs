mod common;

use amarok_lexer::{TokenKind, tokenize};
use common::kinds;

#[test]
fn empty_input_produces_only_end_of_file() {
    let (tokens, diagnostics) = tokenize("");
    assert_eq!(kinds(&tokens), vec![TokenKind::EndOfFile]);
    assert!(diagnostics.is_empty());
}
