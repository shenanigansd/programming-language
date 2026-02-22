use amarok_lexer::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(LiteralValue),
    Grouping(Box<Expression>),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Identifier(Token),
}

impl Expression {
    pub fn kind_name(&self) -> &'static str {
        match self {
            Expression::Literal(_) => "Literal",
            Expression::Grouping(_) => "Grouping",
            Expression::Unary { .. } => "Unary",
            Expression::Binary { .. } => "Binary",
            Expression::Identifier(_) => "Identifier",
        }
    }
}
