use amarok_lexer::Token;
use amarok_parser::parse;
use amarok_syntax::{BinaryOperator, Expression};

#[test]
fn parentheses_around_a_literal_yield_the_literal() {
    // (42)
    let tokens = vec![
        Token::LeftParenthesis,
        Token::NumberLiteral(42.0),
        Token::RightParenthesis,
        Token::EndOfFile,
    ];
    assert_eq!(parse(tokens), Ok(Expression::NumberLiteral(42.0)));
}

#[test]
fn parentheses_override_precedence() {
    // (1 + 2) * 3  must parse as  (1 + 2) * 3, NOT 1 + (2 * 3)
    let tokens = vec![
        Token::LeftParenthesis,
        Token::NumberLiteral(1.0),
        Token::Plus,
        Token::NumberLiteral(2.0),
        Token::RightParenthesis,
        Token::Star,
        Token::NumberLiteral(3.0),
        Token::EndOfFile,
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberLiteral(1.0)),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::NumberLiteral(2.0)),
            }),
            operator: BinaryOperator::Multiply,
            right: Box::new(Expression::NumberLiteral(3.0)),
        }),
    );
}

#[test]
fn nested_parentheses_collapse_to_the_inner_expression() {
    // ((42))
    let tokens = vec![
        Token::LeftParenthesis,
        Token::LeftParenthesis,
        Token::NumberLiteral(42.0),
        Token::RightParenthesis,
        Token::RightParenthesis,
        Token::EndOfFile,
    ];
    assert_eq!(parse(tokens), Ok(Expression::NumberLiteral(42.0)));
}

#[test]
fn a_missing_closing_parenthesis_is_an_error() {
    // (1 + 2   — never closed
    let tokens = vec![
        Token::LeftParenthesis,
        Token::NumberLiteral(1.0),
        Token::Plus,
        Token::NumberLiteral(2.0),
        Token::EndOfFile,
    ];
    let result = parse(tokens);
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains(")"));
}
