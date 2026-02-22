use crate::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
}