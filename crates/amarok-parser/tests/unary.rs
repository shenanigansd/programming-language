mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use amarok_syntax::{Expression, UnaryOperator};
use common::token;

#[test]
fn negation_of_a_number() {
    // -5
    let tokens = vec![
        token(TokenKind::Minus),
        token(TokenKind::NumberLiteral(5.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Unary {
            operator: UnaryOperator::Negate,
            operand: Box::new(Expression::NumberLiteral(5.0)),
        }),
    );
}

#[test]
fn logical_not_of_a_boolean() {
    // not true
    let tokens = vec![
        token(TokenKind::Not),
        token(TokenKind::True),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Unary {
            operator: UnaryOperator::Not,
            operand: Box::new(Expression::BooleanLiteral(true)),
        }),
    );
}

#[test]
fn doubly_nested_negation() {
    // - - 5  (negate the negation of five)
    let tokens = vec![
        token(TokenKind::Minus),
        token(TokenKind::Minus),
        token(TokenKind::NumberLiteral(5.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Unary {
            operator: UnaryOperator::Negate,
            operand: Box::new(Expression::Unary {
                operator: UnaryOperator::Negate,
                operand: Box::new(Expression::NumberLiteral(5.0)),
            }),
        }),
    );
}

#[test]
fn a_bare_literal_still_parses_with_no_unary_operator() {
    // 42 — no operator, so parse_unary falls through to parse_primary.
    let tokens = vec![
        token(TokenKind::NumberLiteral(42.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(parse(tokens), Ok(Expression::NumberLiteral(42.0)));
}
