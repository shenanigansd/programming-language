use amarok_lexer::Token;
use amarok_parser::parse;
use amarok_syntax::{BinaryOperator, Expression};

#[test]
fn equality_of_two_numbers() {
    // 1 == 2
    let tokens = vec![
        Token::NumberLiteral(1.0),
        Token::EqualEqual,
        Token::NumberLiteral(2.0),
        Token::EndOfFile,
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::NumberLiteral(1.0)),
            operator: BinaryOperator::Equal,
            right: Box::new(Expression::NumberLiteral(2.0)),
        }),
    );
}

#[test]
fn comparison_binds_tighter_than_equality() {
    // 1 == 2 < 3  must parse as  1 == (2 < 3)
    let tokens = vec![
        Token::NumberLiteral(1.0),
        Token::EqualEqual,
        Token::NumberLiteral(2.0),
        Token::Less,
        Token::NumberLiteral(3.0),
        Token::EndOfFile,
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::NumberLiteral(1.0)),
            operator: BinaryOperator::Equal,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberLiteral(2.0)),
                operator: BinaryOperator::Less,
                right: Box::new(Expression::NumberLiteral(3.0)),
            }),
        }),
    );
}

#[test]
fn the_whole_ladder_in_one_expression() {
    // 1 == 2 + 3 * 4  must parse as  1 == (2 + (3 * 4))
    let tokens = vec![
        Token::NumberLiteral(1.0),
        Token::EqualEqual,
        Token::NumberLiteral(2.0),
        Token::Plus,
        Token::NumberLiteral(3.0),
        Token::Star,
        Token::NumberLiteral(4.0),
        Token::EndOfFile,
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::Binary {
            left: Box::new(Expression::NumberLiteral(1.0)),
            operator: BinaryOperator::Equal,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberLiteral(2.0)),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::NumberLiteral(3.0)),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(Expression::NumberLiteral(4.0)),
                }),
            }),
        }),
    );
}
