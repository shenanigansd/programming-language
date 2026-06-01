pub use amarok_diagnostics::{Diagnostic, SourcePosition};

use amarok_lexer::Token;
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
        if !matches!(token, Token::EndOfFile) {
            self.current += 1;
        }
        token
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, expected: &Token, message: &str) -> Result<(), Diagnostic> {
        if self.peek() == expected {
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
            |token| match token {
                Token::EqualEqual => Some(BinaryOperator::Equal),
                Token::BangEqual => Some(BinaryOperator::NotEqual),
                _ => None,
            },
            Self::parse_comparison,
        )
    }

    fn parse_comparison(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_left_associative_binary(
            |token| match token {
                Token::Less => Some(BinaryOperator::Less),
                Token::LessEqual => Some(BinaryOperator::LessEqual),
                Token::Greater => Some(BinaryOperator::Greater),
                Token::GreaterEqual => Some(BinaryOperator::GreaterEqual),
                _ => None,
            },
            Self::parse_term,
        )
    }

    fn parse_term(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_left_associative_binary(
            |token| match token {
                Token::Plus => Some(BinaryOperator::Add),
                Token::Minus => Some(BinaryOperator::Subtract),
                _ => None,
            },
            Self::parse_factor,
        )
    }

    fn parse_factor(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_left_associative_binary(
            |token| match token {
                Token::Star => Some(BinaryOperator::Multiply),
                Token::Slash => Some(BinaryOperator::Divide),
                _ => None,
            },
            Self::parse_unary,
        )
    }

    fn parse_unary(&mut self) -> Result<Expression, Diagnostic> {
        let operator = match self.peek() {
            Token::Minus => Some(UnaryOperator::Negate),
            Token::Not => Some(UnaryOperator::Not),
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
        match self.advance() {
            Token::NumberLiteral(value) => Ok(Expression::NumberLiteral(value)),
            Token::StringLiteral(value) => Ok(Expression::StringLiteral(value)),
            Token::True => Ok(Expression::BooleanLiteral(true)),
            Token::False => Ok(Expression::BooleanLiteral(false)),
            Token::Nil => Ok(Expression::NilLiteral),
            Token::LeftParenthesis => {
                let inner = self.parse_expression()?; // recurse to the TOP of the grammar
                self.consume(&Token::RightParenthesis, "expected ')' after expression")?;
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
pub fn parse(tokens: Vec<Token>) -> Result<Expression, Diagnostic> {
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression()?;
    parser.consume(&Token::EndOfFile, "expected end of input after expression")?;
    Ok(expression)
}
