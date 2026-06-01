mod common;

use amarok_lexer::TokenKind;
use amarok_parser::parse;
use amarok_syntax::Expression;
use common::token;

#[test]
fn an_identifier_parses_as_a_variable_reference() {
    let tokens = vec![
        token(TokenKind::Identifier(String::from("x"))),
        token(TokenKind::EndOfFile),
    ];
    assert_eq!(parse(tokens), Ok(Expression::Variable(String::from("x"))));
}
