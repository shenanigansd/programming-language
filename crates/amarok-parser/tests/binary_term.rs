mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use amarok_syntax::{BinaryOperator, Expression, UnaryOperator};
use common::token;

#[test]
fn addition_of_two_numbers() {
    // 1 + 2
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::NumberLiteral(1.0)),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::NumberLiteral(2.0)),
        }),
    );
}

#[test]
fn subtraction_is_left_associative() {
    // 1 - 2 - 3  must parse as  (1 - 2) - 3
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Minus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Minus),
        token(TokenKind::NumberLiteral(3.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberLiteral(1.0)),
                operator: BinaryOperator::Subtract,
                right: Box::new(Expression::NumberLiteral(2.0)),
            }),
            operator: BinaryOperator::Subtract,
            right: Box::new(Expression::NumberLiteral(3.0)),
        }),
    );
}

#[test]
fn binary_minus_and_unary_minus_coexist() {
    // 1 - -2  parses as  1 - (-2)
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Minus),
        token(TokenKind::Minus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::NumberLiteral(1.0)),
            operator: BinaryOperator::Subtract,
            right: Box::new(Expression::Unary {
                operator: UnaryOperator::Negate,
                operand: Box::new(Expression::NumberLiteral(2.0)),
            }),
        }),
    );
}

#[test]
fn a_lone_unary_still_parses_through_the_term_rule() {
    // -5  — no binary operator, so the loop never runs and we get the unary back.
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
