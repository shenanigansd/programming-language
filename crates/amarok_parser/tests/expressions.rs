use amarok_parser::parse_expression;
use amarok_syntax::{BinaryOperator, Expression};

mod common;
use common::strip_spans_expression;

#[test]
fn parses_integer_expression() {
    let expression = parse_expression("123").expect("Expression should parse");
    assert_eq!(expression.value, Expression::Integer(123));
}

#[test]
fn parses_variable_expression() {
    let expression = parse_expression("alpha").expect("Expression should parse");
    assert_eq!(expression.value, Expression::Variable("alpha".to_string()));
}

#[test]
fn parses_string_expression() {
    let expression = parse_expression(r#""hello""#).expect("Expression should parse");
    assert_eq!(expression.value, Expression::String("hello".to_string()));
}

#[test]
fn parses_function_call_no_arguments() {
    let expression = parse_expression("tick()").expect("Expression should parse");

    assert_eq!(
        strip_spans_expression(&expression),
        Expression::FunctionCall {
            name: "tick".to_string(),
            arguments: vec![],
        }
    );
}

#[test]
fn parses_function_call_with_arguments() {
    let expression = parse_expression("print(1, x)").expect("Expression should parse");

    assert_eq!(
        strip_spans_expression(&expression),
        Expression::FunctionCall {
            name: "print".to_string(),
            arguments: vec![
                Expression::Integer(1).into(),
                Expression::Variable("x".to_string()).into(),
            ],
        }
    );
}

#[test]
fn multiplication_has_higher_precedence_than_addition() {
    // a + 2 * 3  =>  a + (2 * 3)
    let expression = parse_expression("a + 2 * 3").expect("Expression should parse");

    assert_eq!(
        strip_spans_expression(&expression),
        Expression::Binary {
            left: Box::new(Expression::Variable("a".to_string()).into()),
            operator: BinaryOperator::Add,
            right: Box::new(
                Expression::Binary {
                    left: Box::new(Expression::Integer(2).into()),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(Expression::Integer(3).into()),
                }
                .into()
            ),
        }
    );
}

#[test]
fn parentheses_override_precedence() {
    // (1 + 2) * 3  =>  (1 + 2) * 3
    let expression = parse_expression("(1 + 2) * 3").expect("Expression should parse");

    assert_eq!(
        strip_spans_expression(&expression),
        Expression::Binary {
            left: Box::new(
                Expression::Binary {
                    left: Box::new(Expression::Integer(1).into()),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Integer(2).into()),
                }
                .into()
            ),
            operator: BinaryOperator::Multiply,
            right: Box::new(Expression::Integer(3).into()),
        }
    );
}
