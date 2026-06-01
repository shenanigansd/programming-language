mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use amarok_syntax::{BinaryOperator, Expression};
use common::token;

#[test]
fn multiplication_of_two_numbers() {
    // 2 * 3
    let tokens = vec![
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Star),
        token(TokenKind::NumberLiteral(3.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::NumberLiteral(2.0)),
            operator: BinaryOperator::Multiply,
            right: Box::new(Expression::NumberLiteral(3.0)),
        }),
    );
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    // 1 + 2 * 3  must parse as  1 + (2 * 3)
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Star),
        token(TokenKind::NumberLiteral(3.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::NumberLiteral(1.0)),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberLiteral(2.0)),
                operator: BinaryOperator::Multiply,
                right: Box::new(Expression::NumberLiteral(3.0)),
            }),
        }),
    );
}

#[test]
fn multiplication_on_the_left_also_binds_tighter() {
    // 2 * 3 + 1  must parse as  (2 * 3) + 1
    let tokens = vec![
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Star),
        token(TokenKind::NumberLiteral(3.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberLiteral(2.0)),
                operator: BinaryOperator::Multiply,
                right: Box::new(Expression::NumberLiteral(3.0)),
            }),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::NumberLiteral(1.0)),
        }),
    );
}

#[test]
fn division_is_left_associative() {
    // 8 / 4 / 2  must parse as  (8 / 4) / 2
    let tokens = vec![
        token(TokenKind::NumberLiteral(8.0)),
        token(TokenKind::Slash),
        token(TokenKind::NumberLiteral(4.0)),
        token(TokenKind::Slash),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberLiteral(8.0)),
                operator: BinaryOperator::Divide,
                right: Box::new(Expression::NumberLiteral(4.0)),
            }),
            operator: BinaryOperator::Divide,
            right: Box::new(Expression::NumberLiteral(2.0)),
        }),
    );
}
