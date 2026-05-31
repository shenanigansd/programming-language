use amarok_lexer::{SourcePosition, Token, tokenize};

#[test]
fn an_unexpected_character_is_skipped_and_recorded() {
    // The @ is invalid, but the parentheses on either side still tokenize.
    let (tokens, diagnostics) = tokenize("(@)");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::EndOfFile,
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
    assert_eq!(tokens, vec![Token::EndOfFile]);
    assert_eq!(diagnostics.len(), 3);
}
