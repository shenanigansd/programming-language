use amarok_interpreter::{Environment, Value, execute_statement};
use amarok_syntax::{BinaryOperator, ExpressionKind, Statement};

mod common;
use common::expression;

#[test]
fn a_let_binds_a_variable_and_yields_no_value() {
    let environment = Environment::new_global();
    let statement = Statement::Let {
        name: String::from("x"),
        initializer: expression(ExpressionKind::NumberLiteral(5.0)),
    };
    assert_eq!(execute_statement(&statement, &environment), Ok(None));
    assert_eq!(environment.borrow().get("x"), Some(Value::Number(5.0)));
}

#[test]
fn an_expression_statement_yields_its_value() {
    let environment = Environment::new_global();
    let statement = Statement::Expression(expression(ExpressionKind::NumberLiteral(42.0)));
    assert_eq!(
        execute_statement(&statement, &environment),
        Ok(Some(Value::Number(42.0))),
    );
}

#[test]
fn a_later_statement_sees_an_earlier_binding() {
    let environment = Environment::new_global();
    execute_statement(
        &Statement::Let {
            name: String::from("x"),
            initializer: expression(ExpressionKind::NumberLiteral(5.0)),
        },
        &environment,
    )
    .unwrap();
    // x + 1
    let result = execute_statement(
        &Statement::Expression(expression(ExpressionKind::Binary {
            left: Box::new(expression(ExpressionKind::Variable(String::from("x")))),
            operator: BinaryOperator::Add,
            right: Box::new(expression(ExpressionKind::NumberLiteral(1.0))),
        })),
        &environment,
    );
    assert_eq!(result, Ok(Some(Value::Number(6.0))));
}
