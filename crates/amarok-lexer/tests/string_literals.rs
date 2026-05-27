use amarok_lexer::{Token, tokenize};

#[test]
fn empty_string_literal() {
    let tokens = tokenize("\"\"");
    assert_eq!(
        tokens,
        vec![Token::StringLiteral(String::new()), Token::EndOfFile],
    );
}

#[test]
fn simple_string_literal() {
    let tokens = tokenize("\"hello\"");
    assert_eq!(
        tokens,
        vec![
            Token::StringLiteral(String::from("hello")),
            Token::EndOfFile,
        ],
    );
}

#[test]
fn string_literal_with_spaces_inside() {
    let tokens = tokenize("\"hello world\"");
    assert_eq!(
        tokens,
        vec![
            Token::StringLiteral(String::from("hello world")),
            Token::EndOfFile,
        ],
    );
}

#[test]
fn string_literal_can_span_multiple_lines() {
    let tokens = tokenize("\"line one\nline two\"");
    assert_eq!(
        tokens,
        vec![
            Token::StringLiteral(String::from("line one\nline two")),
            Token::EndOfFile,
        ],
    );
}

#[test]
fn string_literal_between_other_tokens() {
    let tokens = tokenize("(\"hello\")");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParenthesis,
            Token::StringLiteral(String::from("hello")),
            Token::RightParenthesis,
            Token::EndOfFile,
        ],
    );
}

#[test]
#[should_panic(expected = "unterminated string literal")]
fn unterminated_string_literal_panics() {
    let _ = tokenize("\"this string has no closing quote");
}
