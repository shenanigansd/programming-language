use amarok_lexer::TokenKind;
use amarok_parser::parse_program;
use amarok_syntax::{BinaryOperator, ExpressionKind, Statement};

mod common;
use common::{expression, token};

#[test]
fn a_let_declaration() {
    // let x = 5;
    let tokens = vec![
        token(TokenKind::Let),
        token(TokenKind::Identifier(String::from("x"))),
        token(TokenKind::Equal),
        token(TokenKind::NumberLiteral(5.0)),
        token(TokenKind::Semicolon),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse_program(tokens),
        Ok(vec![Statement::Let {
            name: String::from("x"),
            initializer: expression(ExpressionKind::NumberLiteral(5.0)),
        }]),
    );
}

#[test]
fn an_expression_statement() {
    // 1 + 2;
    let tokens = vec![
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Plus),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Semicolon),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse_program(tokens),
        Ok(vec![Statement::Expression(expression(
            ExpressionKind::Binary {
                left: Box::new(expression(ExpressionKind::NumberLiteral(1.0))),
                operator: BinaryOperator::Add,
                right: Box::new(expression(ExpressionKind::NumberLiteral(2.0))),
            }
        ))]),
    );
}

#[test]
fn several_statements_in_sequence() {
    // let x = 5; x;
    let tokens = vec![
        token(TokenKind::Let),
        token(TokenKind::Identifier(String::from("x"))),
        token(TokenKind::Equal),
        token(TokenKind::NumberLiteral(5.0)),
        token(TokenKind::Semicolon),
        token(TokenKind::Identifier(String::from("x"))),
        token(TokenKind::Semicolon),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(
        parse_program(tokens),
        Ok(vec![
            Statement::Let {
                name: String::from("x"),
                initializer: expression(ExpressionKind::NumberLiteral(5.0)),
            },
            Statement::Expression(expression(ExpressionKind::Variable(String::from("x")))),
        ]),
    );
}

#[test]
fn a_let_missing_its_semicolon_is_an_error() {
    let tokens = vec![
        token(TokenKind::Let),
        token(TokenKind::Identifier(String::from("x"))),
        token(TokenKind::Equal),
        token(TokenKind::NumberLiteral(5.0)),
        token(TokenKind::EndOfFile),
    ];
    assert!(parse_program(tokens).is_err());
}

#[test]
fn a_let_missing_its_name_is_an_error() {
    // let = 5;
    let tokens = vec![
        token(TokenKind::Let),
        token(TokenKind::Equal),
        token(TokenKind::NumberLiteral(5.0)),
        token(TokenKind::Semicolon),
        token(TokenKind::EndOfFile),
    ];
    assert!(parse_program(tokens).is_err());
}

#[test]
fn a_block_groups_its_inner_statements() {
    // { 1; 2; }
    let tokens = vec![
        token(TokenKind::LeftBrace),
        token(TokenKind::NumberLiteral(1.0)),
        token(TokenKind::Semicolon),
        token(TokenKind::NumberLiteral(2.0)),
        token(TokenKind::Semicolon),
        token(TokenKind::RightBrace),
        token(TokenKind::EndOfFile),
    ];
    let program = parse_program(tokens).unwrap();
    assert_eq!(program.len(), 1);
    let Statement::Block(statements) = &program[0] else {
        panic!("expected a block");
    };
    assert_eq!(statements.len(), 2);
}
