use amarok_lexer::{Token, tokenize};

#[test]
fn empty_string_literal() {
    let (tokens, diagnostics) = tokenize("\"\"");
    assert_eq!(
        tokens,
        vec![Token::StringLiteral(String::new()), Token::EndOfFile],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn simple_string_literal() {
    let (tokens, diagnostics) = tokenize("\"hello\"");
    assert_eq!(
        tokens,
        vec![
            Token::StringLiteral(String::from("hello")),
            Token::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn string_literal_with_spaces_inside() {
    let (tokens, diagnostics) = tokenize("\"hello world\"");
    assert_eq!(
        tokens,
        vec![
            Token::StringLiteral(String::from("hello world")),
            Token::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn string_literal_can_span_multiple_lines() {
    let (tokens, diagnostics) = tokenize("\"line one\nline two\"");
    assert_eq!(
        tokens,
        vec![
            Token::StringLiteral(String::from("line one\nline two")),
            Token::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn string_literal_between_other_tokens() {
    let (tokens, diagnostics) = tokenize("(\"hello\")");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParenthesis,
            Token::StringLiteral(String::from("hello")),
            Token::RightParenthesis,
            Token::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn unterminated_string_literal_records_a_diagnostic() {
    let (tokens, diagnostics) = tokenize("\"this string has no closing quote");
    assert_eq!(
        tokens,
        vec![
            Token::StringLiteral(String::from("this string has no closing quote")),
            Token::EndOfFile,
        ],
    );
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].message, "unterminated string literal");
}
