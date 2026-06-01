mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use amarok_syntax::{BinaryOperator, ExpressionKind};
use common::{expression, token};

#[test]
fn equality_of_two_numbers() {
    // 1 == 2
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::EqualEqual),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(expression(ExpressionKind::Binary {
            left: Box::new(expression(ExpressionKind::NumberLiteral(1.0))),
            operator: BinaryOperator::Equal,
            right: Box::new(expression(ExpressionKind::NumberLiteral(2.0))),
        })),
    );
}

#[test]
fn comparison_binds_tighter_than_equality() {
    // 1 == 2 < 3  must parse as  1 == (2 < 3)
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::EqualEqual),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Less),
        token(TokenKind::NumberLiteral(3.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(expression(ExpressionKind::Binary {
            left: Box::new(expression(ExpressionKind::NumberLiteral(1.0))),
            operator: BinaryOperator::Equal,
            right: Box::new(expression(ExpressionKind::Binary {
                left: Box::new(expression(ExpressionKind::NumberLiteral(2.0))),
                operator: BinaryOperator::Less,
                right: Box::new(expression(ExpressionKind::NumberLiteral(3.0))),
            })),
        })),
    );
}

#[test]
fn the_whole_ladder_in_one_expression() {
    // 1 == 2 + 3 * 4  must parse as  1 == (2 + (3 * 4))
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::EqualEqual),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(3.0)),
        token(TokenKind::Star),
        token(TokenKind::NumberLiteral(4.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(expression(ExpressionKind::Binary {
            left: Box::new(expression(ExpressionKind::NumberLiteral(1.0))),
            operator: BinaryOperator::Equal,
            right: Box::new(expression(ExpressionKind::Binary {
                left: Box::new(expression(ExpressionKind::NumberLiteral(2.0))),
                operator: BinaryOperator::Add,
                right: Box::new(expression(ExpressionKind::Binary {
                    left: Box::new(expression(ExpressionKind::NumberLiteral(3.0))),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(expression(ExpressionKind::NumberLiteral(4.0))),
                })),
            })),
        })),
    );
}
