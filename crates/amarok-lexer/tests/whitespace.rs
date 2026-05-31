use amarok_lexer::{Token, tokenize};

#[test]
fn spaces_between_tokens_are_skipped() {
    let (tokens, diagnostics) = tokenize("  (  )  ");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn tabs_and_newlines_and_carriage_returns_are_also_skipped() {
    let (tokens, diagnostics) = tokenize("(\n\t\r)");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn input_of_only_whitespace_produces_only_end_of_file() {
    let (tokens, diagnostics) = tokenize("   \n\t  ");
    assert_eq!(tokens, vec![Token::EndOfFile]);
    assert!(diagnostics.is_empty());
}
