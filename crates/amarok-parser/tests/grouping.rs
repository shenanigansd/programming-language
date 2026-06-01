mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use amarok_syntax::{BinaryOperator, ExpressionKind};
use common::{expression, token};

#[test]
fn parentheses_around_a_literal_yield_the_literal() {
    // (42)
    let tokens = vec![
        token(TokenKind::LeftParenthesis),
        token(TokenKind::NumberLiteral(42.0)),
        token(TokenKind::RightParenthesis),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(expression(ExpressionKind::NumberLiteral(42.0))),
    );
}

#[test]
fn parentheses_override_precedence() {
    // (1 + 2) * 3  must parse as  (1 + 2) * 3, NOT 1 + (2 * 3)
    let tokens = vec![
        token(TokenKind::LeftParenthesis),
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::RightParenthesis),
        token(TokenKind::Star),
        token(TokenKind::NumberLiteral(3.0)),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(expression(ExpressionKind::Binary {
            left: Box::new(expression(ExpressionKind::Binary {
                left: Box::new(expression(ExpressionKind::NumberLiteral(1.0))),
                operator: BinaryOperator::Add,
                right: Box::new(expression(ExpressionKind::NumberLiteral(2.0))),
            })),
            operator: BinaryOperator::Multiply,
            right: Box::new(expression(ExpressionKind::NumberLiteral(3.0))),
        })),
    );
}

#[test]
fn nested_parentheses_collapse_to_the_inner_expression() {
    // ((42))
    let tokens = vec![
        token(TokenKind::LeftParenthesis),
        token(TokenKind::LeftParenthesis),
        token(TokenKind::NumberLiteral(42.0)),
        token(TokenKind::RightParenthesis),
        token(TokenKind::RightParenthesis),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse(tokens),
        Ok(expression(ExpressionKind::NumberLiteral(42.0))),
    );
}

#[test]
fn a_missing_closing_parenthesis_is_an_error() {
    // (1 + 2   — never closed
    let tokens = vec![
        token(TokenKind::LeftParenthesis),
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::EndOfFile),
    ];
    let result = parse(tokens);
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains(')'));
}
