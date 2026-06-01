use amarok_interpreter::Value;
use amarok_syntax::{BinaryOperator, Expression, ExpressionKind};
mod common;
use common::{eval, expression};

fn number(value: f64) -> Expression {
    expression(ExpressionKind::NumberLiteral(value))
}

fn binary(left: Expression, operator: BinaryOperator, right: Expression) -> Expression {
    expression(ExpressionKind::Binary {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })
}

#[test]
fn addition_of_numbers() {
    assert_eq!(
        eval(&binary(number(1.0), BinaryOperator::Add, number(2.0))),
        Ok(Value::Number(3.0)),
    );
}

#[test]
fn string_concatenation() {
    let expression = binary(
        expression(ExpressionKind::StringLiteral(String::from("foo"))),
        BinaryOperator::Add,
        expression(ExpressionKind::StringLiteral(String::from("bar"))),
    );
    assert_eq!(eval(&expression), Ok(Value::String(String::from("foobar"))));
}

#[test]
fn adding_a_number_and_a_string_is_a_runtime_error() {
    let expression = binary(
        number(1.0),
        BinaryOperator::Add,
        expression(ExpressionKind::StringLiteral(String::from("x"))),
    );
    assert!(eval(&expression).is_err());
}

#[test]
fn the_precedence_tree_evaluates_to_the_right_number() {
    // 1 + 2 * 3  →  7   (the tree 1 + (2 * 3))
    let expression = binary(
        number(1.0),
        BinaryOperator::Add,
        binary(number(2.0), BinaryOperator::Multiply, number(3.0)),
    );
    assert_eq!(eval(&expression), Ok(Value::Number(7.0)));
}

#[test]
fn division_by_zero_is_a_runtime_error() {
    assert!(eval(&binary(number(1.0), BinaryOperator::Divide, number(0.0))).is_err());
}

#[test]
fn less_than_compares_two_numbers() {
    assert_eq!(
        eval(&binary(number(1.0), BinaryOperator::Less, number(2.0))),
        Ok(Value::Boolean(true)),
    );
}

#[test]
fn equality_across_different_types_is_false_not_an_error() {
    // 1 == "1"  →  false
    let expression = binary(
        number(1.0),
        BinaryOperator::Equal,
        expression(ExpressionKind::StringLiteral(String::from("1"))),
    );
    assert_eq!(eval(&expression), Ok(Value::Boolean(false)));
}

#[test]
fn equality_of_two_equal_numbers_is_true() {
    assert_eq!(
        eval(&binary(number(2.0), BinaryOperator::Equal, number(2.0))),
        Ok(Value::Boolean(true)),
    );
}
