use amarok_interpreter::{Value, evaluate};
use amarok_syntax::{Expression, UnaryOperator};

#[test]
fn a_number_literal_evaluates_to_a_number() {
    assert_eq!(
        evaluate(&Expression::NumberLiteral(42.0)),
        Ok(Value::Number(42.0))
    );
}

#[test]
fn a_string_literal_evaluates_to_a_string() {
    let expression = Expression::StringLiteral(String::from("hello"));
    assert_eq!(
        evaluate(&expression),
        Ok(Value::String(String::from("hello")))
    );
}

#[test]
fn negation_negates_a_number() {
    // -5
    let expression = Expression::Unary {
        operator: UnaryOperator::Negate,
        operand: Box::new(Expression::NumberLiteral(5.0)),
    };
    assert_eq!(evaluate(&expression), Ok(Value::Number(-5.0)));
}

#[test]
fn not_of_true_is_false() {
    let expression = Expression::Unary {
        operator: UnaryOperator::Not,
        operand: Box::new(Expression::BooleanLiteral(true)),
    };
    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
fn not_of_nil_is_true_because_nil_is_falsy() {
    let expression = Expression::Unary {
        operator: UnaryOperator::Not,
        operand: Box::new(Expression::NilLiteral),
    };
    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
fn not_of_a_number_is_false_because_numbers_are_truthy() {
    // not 5  →  false
    let expression = Expression::Unary {
        operator: UnaryOperator::Not,
        operand: Box::new(Expression::NumberLiteral(5.0)),
    };
    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
fn negating_a_string_is_a_runtime_error() {
    // -"hello"
    let expression = Expression::Unary {
        operator: UnaryOperator::Negate,
        operand: Box::new(Expression::StringLiteral(String::from("hello"))),
    };
    assert!(evaluate(&expression).is_err());
}

#[test]
fn double_negation_evaluates_through_both_layers() {
    // - - 5  →  5
    let expression = Expression::Unary {
        operator: UnaryOperator::Negate,
        operand: Box::new(Expression::Unary {
            operator: UnaryOperator::Negate,
            operand: Box::new(Expression::NumberLiteral(5.0)),
        }),
    };
    assert_eq!(evaluate(&expression), Ok(Value::Number(5.0)));
}
