mod common;

use amarok_lexer::{TokenKind, tokenize};
use common::kinds;

#[test]
fn simple_identifier() {
    let (tokens, diagnostics) = tokenize("hello");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Identifier(String::from("hello")),
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn identifier_can_contain_digits_after_the_first_character() {
    let (tokens, diagnostics) = tokenize("var1");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Identifier(String::from("var1")),
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn identifier_can_contain_underscores() {
    let (tokens, diagnostics) = tokenize("my_variable");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Identifier(String::from("my_variable")),
            TokenKind::EndOfFile
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn a_lone_underscore_is_an_identifier() {
    let (tokens, diagnostics) = tokenize("_");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Identifier(String::from("_")),
            TokenKind::EndOfFile
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn identifier_cannot_start_with_a_digit() {
    // "1foo" should be a number followed by an identifier, not one token.
    let (tokens, diagnostics) = tokenize("1foo");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::NumberLiteral(1.0),
            TokenKind::Identifier(String::from("foo")),
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn all_keywords_produce_their_keyword_token() {
    let cases = [
        ("and", TokenKind::And),
        ("or", TokenKind::Or),
        ("not", TokenKind::Not),
        ("if", TokenKind::If),
        ("else", TokenKind::Else),
        ("while", TokenKind::While),
        ("for", TokenKind::For),
        ("let", TokenKind::Let),
        ("fun", TokenKind::Fun),
        ("return", TokenKind::Return),
        ("true", TokenKind::True),
        ("false", TokenKind::False),
        ("nil", TokenKind::Nil),
    ];
    for (source, expected) in cases {
        let (tokens, diagnostics) = tokenize(source);
        assert_eq!(
            kinds(&tokens),
            vec![expected, TokenKind::EndOfFile],
            "keyword {source:?} did not produce the expected token",
        );
        assert!(diagnostics.is_empty());
    }
}

#[test]
fn keyword_prefix_followed_by_other_letters_is_an_identifier() {
    // "ifx" starts with "if" but it should be the single identifier "ifx",
    // not Token::If followed by Token::Identifier("x").
    let (tokens, diagnostics) = tokenize("ifx");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Identifier(String::from("ifx")),
            TokenKind::EndOfFile
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn identifier_containing_a_keyword_as_a_substring_is_one_identifier() {
    let (tokens, diagnostics) = tokenize("x_if_y");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Identifier(String::from("x_if_y")),
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}
