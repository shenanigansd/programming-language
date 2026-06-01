#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NilLiteral,
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    /// A binary operation with a left operand, an operator, and a right operand,
    /// such as `1 + 2`.
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    /// A reference to a variable by name, such as `x`.
    Variable(String),
}

/// An operator that takes a single operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Arithmetic negation, written `-`.
    Negate,
    /// Logical negation, written `not`.
    Not,
}

/// An operator that takes two operands. More variants (multiplication,
/// comparison, equality) will join these as we add precedence levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

/// A top-level instruction in a program.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// A variable declaration: `let name = initializer ;`.
    Let {
        name: String,
        initializer: Expression,
    },
    /// An expression run as a statement: `expression ;`.
    Expression(Expression),
}
