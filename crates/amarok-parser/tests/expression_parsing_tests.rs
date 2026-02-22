use amarok_lexer::lexer::Lexer;
use amarok_parser::expression::{Expression, LiteralValue};
use amarok_parser::parser::Parser;

fn parse_expression(source: &str) -> Expression {
    let tokens = Lexer::new(source).scan_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse_expression().unwrap()
}

#[test]
fn parses_number_literal() {
    let expression = parse_expression("123");
    match expression {
        Expression::Literal(LiteralValue::Number(value)) => assert_eq!(value, 123.0),
        other => panic!("Expected number literal, got {:?}", other),
    }
}

#[test]
fn parses_grouping_and_precedence() {
    let expression = parse_expression("(1 + 2) * 3");

    // should be: (* (group (+ 1 2)) 3)
    match expression {
        Expression::Binary { operator, .. } => {
            assert_eq!(operator.lexeme, "*");
        }
        other => panic!("Expected binary expression, got {:?}", other),
    }
}

#[test]
fn parses_unary() {
    let expression = parse_expression("-123");
    match expression {
        Expression::Unary { operator, .. } => assert_eq!(operator.lexeme, "-"),
        other => panic!("Expected unary expression, got {:?}", other),
    }
}

#[test]
fn parses_equality_chain() {
    let expression = parse_expression("1 == 2 != 3");
    match expression {
        Expression::Binary { operator, .. } => {
            // left-associative: ((1 == 2) != 3)
            assert_eq!(operator.lexeme, "!=");
        }
        other => panic!("Expected binary expression, got {:?}", other),
    }
}
