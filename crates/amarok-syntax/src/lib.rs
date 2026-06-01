use amarok_diagnostics::SourcePosition;

/// An expression node: its shape paired with the source position used to point a
/// caret if evaluating this node fails at runtime. The position is the node's
/// most error-relevant token — a literal/variable points at itself; a unary or
/// binary operation points at its operator.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub position: SourcePosition,
}

/// The shape of an expression, independent of where it sits in the source.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
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

impl ExpressionKind {
    /// Attach a source position, producing a full expression node. Lets callers
    /// build nodes fluently, e.g. `ExpressionKind::NumberLiteral(5.0).at(position)`.
    #[allow(clippy::must_use_candidate)]
    pub fn at(self, position: SourcePosition) -> Expression {
        Expression {
            kind: self,
            position,
        }
    }
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
