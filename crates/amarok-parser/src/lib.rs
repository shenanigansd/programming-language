pub use amarok_diagnostics::{Diagnostic, SourcePosition};

use amarok_lexer::{Token, TokenKind};
use amarok_syntax::{BinaryOperator, Expression, UnaryOperator};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        // Never step past the end-of-file sentinel; it's always the last token.
        if !matches!(token.kind, TokenKind::EndOfFile) {
            self.current += 1;
        }
        token
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, expected: &TokenKind, message: &str) -> Result<(), Diagnostic> {
        if &self.peek().kind == expected {
            self.advance();
            Ok(())
        } else {
            Err(Diagnostic::new(
                message,
                // Placeholder position, same as our other parser errors for now.
                SourcePosition { character_index: 0 },
            ))
        }
    }

    fn parse_left_associative_binary(
        &mut self,
        match_operator: fn(&Token) -> Option<BinaryOperator>,
        parse_operand: fn(&mut Self) -> Result<Expression, Diagnostic>,
    ) -> Result<Expression, Diagnostic> {
        let mut left = parse_operand(self)?;
        while let Some(operator) = match_operator(self.peek()) {
            self.advance();
            let right = parse_operand(self)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_left_associative_binary(
            |token| match token.kind {
                TokenKind::EqualEqual => Some(BinaryOperator::Equal),
                TokenKind::BangEqual => Some(BinaryOperator::NotEqual),
                _ => None,
            },
            Self::parse_comparison,
        )
    }

    fn parse_comparison(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_left_associative_binary(
            |token| match token.kind {
                TokenKind::Less => Some(BinaryOperator::Less),
                TokenKind::LessEqual => Some(BinaryOperator::LessEqual),
                TokenKind::Greater => Some(BinaryOperator::Greater),
                TokenKind::GreaterEqual => Some(BinaryOperator::GreaterEqual),
                _ => None,
            },
            Self::parse_term,
        )
    }

    fn parse_term(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_left_associative_binary(
            |token| match token.kind {
                TokenKind::Plus => Some(BinaryOperator::Add),
                TokenKind::Minus => Some(BinaryOperator::Subtract),
                _ => None,
            },
            Self::parse_factor,
        )
    }

    fn parse_factor(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_left_associative_binary(
            |token| match token.kind {
                TokenKind::Star => Some(BinaryOperator::Multiply),
                TokenKind::Slash => Some(BinaryOperator::Divide),
                _ => None,
            },
            Self::parse_unary,
        )
    }

    fn parse_unary(&mut self) -> Result<Expression, Diagnostic> {
        let operator = match self.peek().kind {
            TokenKind::Minus => Some(UnaryOperator::Negate),
            TokenKind::Not => Some(UnaryOperator::Not),
            _ => None,
        };
        match operator {
            Some(operator) => {
                self.advance(); // consume the operator token
                let operand = self.parse_unary()?; // recurse for the operand
                Ok(Expression::Unary {
                    operator,
                    operand: Box::new(operand),
                })
            }
            None => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, Diagnostic> {
        match self.advance().kind {
            TokenKind::NumberLiteral(value) => Ok(Expression::NumberLiteral(value)),
            TokenKind::StringLiteral(value) => Ok(Expression::StringLiteral(value)),
            TokenKind::True => Ok(Expression::BooleanLiteral(true)),
            TokenKind::False => Ok(Expression::BooleanLiteral(false)),
            TokenKind::Nil => Ok(Expression::NilLiteral),
            TokenKind::LeftParenthesis => {
                let inner = self.parse_expression()?; // recurse to the TOP of the grammar
                self.consume(
                    &TokenKind::RightParenthesis,
                    "expected ')' after expression",
                )?;
                Ok(inner)
            }
            unexpected => Err(Diagnostic::new(
                format!("expected an expression, found {unexpected:?}"),
                SourcePosition { character_index: 0 },
            )),
        }
    }
}

/// Parse a token stream into a single expression.
///
/// # Errors
///
/// Returns a [`Diagnostic`] if the input token stream is not a valid expression
/// or if trailing tokens remain after parsing.
pub fn parse(tokens: Vec<Token>) -> Result<Expression, Diagnostic> {
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression()?;
    parser.consume(
        &TokenKind::EndOfFile,
        "expected end of input after expression",
    )?;
    Ok(expression)
}
