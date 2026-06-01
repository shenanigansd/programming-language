mod common;

use amarok_lexer::{SourcePosition, TokenKind, tokenize};
use common::kinds;

#[test]
fn an_unexpected_character_is_skipped_and_recorded() {
    // The @ is invalid, but the parentheses on either side still tokenize.
    let (tokens, diagnostics) = tokenize("(@)");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::LeftParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::EndOfFile,
        ],
    );
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].position,
        SourcePosition { character_index: 1 }
    );
}

#[test]
fn multiple_unexpected_characters_each_get_their_own_diagnostic() {
    let (tokens, diagnostics) = tokenize("@#$");
    assert_eq!(kinds(&tokens), vec![TokenKind::EndOfFile]);
    assert_eq!(diagnostics.len(), 3);
}
