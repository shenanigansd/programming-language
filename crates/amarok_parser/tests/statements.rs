use amarok_parser::parse_program;
use amarok_syntax::{BinaryOperator, Expression, Program, Statement};

mod common;
use common::strip_spans_program;

#[test]
fn parses_assignment_statement() {
    let program = parse_program("x = 1 + 2;").expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(
        program,
        Program {
            statements: vec![
                Statement::Assignment {
                    name: "x".to_string(),
                    value: Expression::Binary {
                        left: Box::new(Expression::Integer(1).into()),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Integer(2).into()),
                    }
                    .into(),
                }
                .into(),
            ],
        }
    );
}

#[test]
fn parses_expression_statement_function_call() {
    let program = parse_program("print(123);").expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(program.statements.len(), 1);

    assert_eq!(
        program.statements[0].value,
        Statement::Expression {
            expression: Expression::FunctionCall {
                path: vec!["print".to_string()],
                arguments: vec![Expression::Integer(123).into()],
            }
            .into(),
        }
    );
}

#[test]
fn parses_block_statement() {
    let program = parse_program("{ x = 1; print(x); }").expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(program.statements.len(), 1);

    let Statement::Block { statements } = &program.statements[0].value else {
        panic!("Expected a block statement.");
    };

    assert_eq!(statements.len(), 2);

    assert!(matches!(statements[0].value, Statement::Assignment { .. }));
    assert!(matches!(statements[1].value, Statement::Expression { .. }));
}

#[test]
fn parses_if_else_statement() {
    let program =
        parse_program("if (x) { print(1); } else { print(2); }").expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(program.statements.len(), 1);

    let Statement::If {
        condition,
        then_branch,
        else_branch,
    } = &program.statements[0].value
    else {
        panic!("Expected an if statement.");
    };

    assert_eq!(condition.value, Expression::Variable("x".to_string()));
    assert_eq!(then_branch.len(), 1);
    assert_eq!(else_branch.len(), 1);
}

#[test]
fn parses_while_statement() {
    let program = parse_program("while (x) { x = x - 1; }").expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(program.statements.len(), 1);

    let Statement::While { condition, body } = &program.statements[0].value else {
        panic!("Expected a while statement.");
    };

    assert_eq!(condition.value, Expression::Variable("x".to_string()));
    assert_eq!(body.len(), 1);
}

#[test]
fn parses_function_definition_and_return() {
    let source = r#"
        def add(a, b) { return a + b; }
    "#;

    let program = parse_program(source).expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(program.statements.len(), 1);

    let Statement::FunctionDefinition {
        name,
        parameters,
        body,
    } = &program.statements[0].value
    else {
        panic!("Expected a function definition.");
    };

    assert_eq!(name, "add");
    assert_eq!(parameters, &vec!["a".to_string(), "b".to_string()]);
    assert_eq!(body.len(), 1);

    let Statement::Return { value } = &body[0].value else {
        panic!("Expected return inside function body.");
    };

    let value = value.as_ref().expect("Return should have an expression");
    assert!(matches!(value.value, Expression::Binary { .. }));
}

#[test]
fn parses_single_segment_use() {
    let program = parse_program("use foo;").expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(
        program,
        Program {
            statements: vec![
                Statement::Use {
                    path: vec!["foo".to_string()],
                }
                .into(),
            ],
        }
    );
}

#[test]
fn parses_multi_segment_use() {
    let program = parse_program("use std::math;").expect("Program should parse");
    let program = strip_spans_program(&program);

    assert_eq!(
        program,
        Program {
            statements: vec![
                Statement::Use {
                    path: vec!["std".to_string(), "math".to_string()],
                }
                .into(),
            ],
        }
    );
}

#[test]
fn use_without_semicolon_fails() {
    assert!(parse_program("use foo").is_err());
}
