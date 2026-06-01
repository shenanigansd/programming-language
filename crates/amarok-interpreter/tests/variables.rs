use amarok_interpreter::{Environment, Value, evaluate};
use amarok_syntax::{BinaryOperator, Expression};

#[test]
fn a_variable_reads_its_value_from_the_environment() {
    let mut environment = Environment::new();
    environment.define("x", Value::Number(5.0));
    let expression = Expression::Variable(String::from("x"));
    assert_eq!(evaluate(&expression, &environment), Ok(Value::Number(5.0)));
}

#[test]
fn an_undefined_variable_is_a_runtime_error() {
    let environment = Environment::new();
    let expression = Expression::Variable(String::from("missing"));
    assert!(evaluate(&expression, &environment).is_err());
}

#[test]
fn a_variable_can_be_used_inside_an_expression() {
    let mut environment = Environment::new();
    environment.define("x", Value::Number(10.0));
    // x + 1
    let expression = Expression::Binary {
        left: Box::new(Expression::Variable(String::from("x"))),
        operator: BinaryOperator::Add,
        right: Box::new(Expression::NumberLiteral(1.0)),
    };
    assert_eq!(evaluate(&expression, &environment), Ok(Value::Number(11.0)));
}
