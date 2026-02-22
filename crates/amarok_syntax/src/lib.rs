//! Core syntax structures for the Amarok language.
//!
//! This crate defines the Abstract Syntax Tree (AST).
//! It contains NO parsing and NO execution logic.

use std::fmt;
mod span;
pub use span::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Spanned<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        name: String,
        value: Spanned<Expression>,
    },
    Expression {
        expression: Spanned<Expression>,
    },
    Block {
        statements: Vec<Spanned<Statement>>,
    },
    If {
        condition: Spanned<Expression>,
        then_branch: Vec<Spanned<Statement>>,
        else_branch: Vec<Spanned<Statement>>,
    },
    While {
        condition: Spanned<Expression>,
        body: Vec<Spanned<Statement>>,
    },
    FunctionDefinition {
        name: String,
        parameters: Vec<String>,
        body: Vec<Spanned<Statement>>,
    },
    Return {
        value: Option<Spanned<Expression>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Spanned<Expression>>,
        operator: BinaryOperator,
        right: Box<Spanned<Expression>>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Spanned<Expression>>,
    },
}
/// Supported binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
        };
        write!(formatter, "{symbol}")
    }
}
