mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use amarok_syntax::Expression;
use common::token;

#[test]
fn parses_a_number_literal() {
    let tokens = vec![
        token(TokenKind::NumberLiteral(42.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(parse(tokens), Ok(Expression::NumberLiteral(42.0)));
}

#[test]
fn parses_a_string_literal() {
    let tokens = vec![
        token(TokenKind::StringLiteral(String::from("hello"))),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::StringLiteral(String::from("hello"))),
    );
}

#[test]
fn parses_the_true_literal() {
    let tokens = vec![token(TokenKind::True), token(TokenKind::EndOfFile)];
    assert_eq!(parse(tokens), Ok(Expression::BooleanLiteral(true)));
}

#[test]
fn parses_the_false_literal() {
    let tokens = vec![token(TokenKind::False), token(TokenKind::EndOfFile)];
    assert_eq!(parse(tokens), Ok(Expression::BooleanLiteral(false)));
}

#[test]
fn parses_the_nil_literal() {
    let tokens = vec![token(TokenKind::Nil), token(TokenKind::EndOfFile)];
    assert_eq!(parse(tokens), Ok(Expression::NilLiteral));
}

#[test]
fn an_empty_token_stream_is_an_error() {
    let tokens = vec![token(TokenKind::EndOfFile)];
    assert!(parse(tokens).is_err());
}
