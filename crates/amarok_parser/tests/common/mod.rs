//! Test helpers shared between integration tests in this crate.
//!
//! Cargo treats `tests/common/mod.rs` as a module rather than a separate
//! integration test binary, so importing `mod common;` from each test file
//! pulls in these helpers without producing an empty test target.

use amarok_syntax::{Expression, Program, Spanned, Statement};

#[allow(dead_code)]
pub fn strip_spans_expression(expression: &Spanned<Expression>) -> Expression {
    match &expression.value {
        Expression::Integer(value) => Expression::Integer(*value),
        Expression::String(value) => Expression::String(value.clone()),
        Expression::Variable(name) => Expression::Variable(name.clone()),

        Expression::FunctionCall { path, arguments } => Expression::FunctionCall {
            path: path.clone(),
            arguments: arguments
                .iter()
                .map(strip_spans_expression)
                .map(Spanned::from)
                .collect(),
        },

        Expression::Binary {
            left,
            operator,
            right,
        } => Expression::Binary {
            left: Box::new(Spanned::from(strip_spans_expression(left))),
            operator: *operator,
            right: Box::new(Spanned::from(strip_spans_expression(right))),
        },
    }
}

#[allow(dead_code)]
pub fn strip_spans_statement(statement: &Spanned<Statement>) -> Statement {
    match &statement.value {
        Statement::Assignment { name, value } => Statement::Assignment {
            name: name.clone(),
            value: Spanned::from(strip_spans_expression(value)),
        },

        Statement::Expression { expression } => Statement::Expression {
            expression: Spanned::from(strip_spans_expression(expression)),
        },

        Statement::Block { statements } => Statement::Block {
            statements: statements
                .iter()
                .map(strip_spans_statement)
                .map(Spanned::from)
                .collect(),
        },

        Statement::If {
            condition,
            then_branch,
            else_branch,
        } => Statement::If {
            condition: Spanned::from(strip_spans_expression(condition)),
            then_branch: then_branch
                .iter()
                .map(strip_spans_statement)
                .map(Spanned::from)
                .collect(),
            else_branch: else_branch
                .iter()
                .map(strip_spans_statement)
                .map(Spanned::from)
                .collect(),
        },

        Statement::While { condition, body } => Statement::While {
            condition: Spanned::from(strip_spans_expression(condition)),
            body: body
                .iter()
                .map(strip_spans_statement)
                .map(Spanned::from)
                .collect(),
        },

        Statement::FunctionDefinition {
            name,
            parameters,
            body,
        } => Statement::FunctionDefinition {
            name: name.clone(),
            parameters: parameters.clone(),
            body: body
                .iter()
                .map(strip_spans_statement)
                .map(Spanned::from)
                .collect(),
        },

        Statement::Return { value } => Statement::Return {
            value: value
                .as_ref()
                .map(strip_spans_expression)
                .map(Spanned::from),
        },
    }
}

#[allow(dead_code)]
pub fn strip_spans_program(program: &Program) -> Program {
    Program {
        statements: program
            .statements
            .iter()
            .map(strip_spans_statement)
            .map(Spanned::from)
            .collect(),
    }
}
