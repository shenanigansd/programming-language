use amarok_interpreter::{Environment, Value, execute_statement};
use amarok_syntax::{BinaryOperator, Expression, Statement};

#[test]
fn a_let_binds_a_variable_and_yields_no_value() {
    let mut environment = Environment::new();
    let statement = Statement::Let {
        name: String::from("x"),
        initializer: Expression::NumberLiteral(5.0),
    };
    assert_eq!(execute_statement(&statement, &mut environment), Ok(None));
    assert_eq!(environment.get("x"), Some(&Value::Number(5.0)));
}

#[test]
fn an_expression_statement_yields_its_value() {
    let mut environment = Environment::new();
    let statement = Statement::Expression(Expression::NumberLiteral(42.0));
    assert_eq!(
        execute_statement(&statement, &mut environment),
        Ok(Some(Value::Number(42.0))),
    );
}

#[test]
fn a_later_statement_sees_an_earlier_binding() {
    let mut environment = Environment::new();
    execute_statement(
        &Statement::Let {
            name: String::from("x"),
            initializer: Expression::NumberLiteral(5.0),
        },
        &mut environment,
    )
    .unwrap();
    // x + 1
    let result = execute_statement(
        &Statement::Expression(Expression::Binary {
            left: Box::new(Expression::Variable(String::from("x"))),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::NumberLiteral(1.0)),
        }),
        &mut environment,
    );
    assert_eq!(result, Ok(Some(Value::Number(6.0))));
}
