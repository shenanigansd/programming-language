mod common;

use amarok_lexer::{TokenKind, tokenize};
use common::kinds;

#[test]
fn single_digit_integer() {
    let (tokens, diagnostics) = tokenize("5");
    assert_eq!(
        kinds(&tokens),
        vec![TokenKind::NumberLiteral(5.0), TokenKind::EndOfFile],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn multi_digit_integer() {
    let (tokens, diagnostics) = tokenize("12345");
    assert_eq!(
        kinds(&tokens),
        vec![TokenKind::NumberLiteral(12345.0), TokenKind::EndOfFile],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn zero_on_its_own() {
    let (tokens, diagnostics) = tokenize("0");
    assert_eq!(
        kinds(&tokens),
        vec![TokenKind::NumberLiteral(0.0), TokenKind::EndOfFile],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn decimal_number() {
    let (tokens, diagnostics) = tokenize("3.14");
    assert_eq!(
        kinds(&tokens),
        vec![TokenKind::NumberLiteral(3.14), TokenKind::EndOfFile],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn integer_followed_by_dot_with_nothing_after_is_two_tokens() {
    let (tokens, diagnostics) = tokenize("123.");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::NumberLiteral(123.0),
            TokenKind::Dot,
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn dot_followed_by_digits_is_two_tokens() {
    let (tokens, diagnostics) = tokenize(".5");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Dot,
            TokenKind::NumberLiteral(5.0),
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn number_inside_a_parenthesized_addition() {
    let (tokens, diagnostics) = tokenize("(1 + 2.5)");
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::LeftParenthesis,
            TokenKind::NumberLiteral(1.0),
            TokenKind::Plus,
            TokenKind::NumberLiteral(2.5),
            TokenKind::RightParenthesis,
            TokenKind::EndOfFile,
        ],
    );
    assert!(diagnostics.is_empty());
}
