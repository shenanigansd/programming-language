use crate::expression::{Expression, LiteralValue};
use crate::statement::Statement;
use amarok_lexer::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line_number: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, line_number: usize) -> Self {
        Self {
            message: message.into(),
            line_number,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_equality()
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )*
    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.parse_comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.parse_comparison()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.parse_term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // term -> factor ( ( "-" | "+" ) factor )*
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.parse_factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.parse_factor()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // factor -> unary ( ( "/" | "*" ) unary )*
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.parse_unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // unary -> ( "!" | "-" ) unary | primary
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.parse_primary()
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | IDENTIFIER | "(" expression ")"
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expression::Literal(LiteralValue::Boolean(false)));
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expression::Literal(LiteralValue::Boolean(true)));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expression::Literal(LiteralValue::Nil));
        }

        if self.matches(&[TokenType::Number]) {
            let token = self.previous();
            let number_value: f64 = token.lexeme.parse().map_err(|_| {
                ParseError::new(
                    format!("Invalid number literal: {}", token.lexeme),
                    token.line_number,
                )
            })?;
            return Ok(Expression::Literal(LiteralValue::Number(number_value)));
        }

        if self.matches(&[TokenType::String]) {
            // Right now lexer includes quotes in lexeme: "\"hello\""
            // For now, strip a single leading and trailing quote if present.
            let token = self.previous();
            let mut text = token.lexeme.clone();
            if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
                text = text[1..text.len() - 1].to_string();
            }
            return Ok(Expression::Literal(LiteralValue::String(text)));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expression::Identifier(self.previous().clone()));
        }

        if self.matches(&[TokenType::LeftParenthesis]) {
            let expression = self.parse_expression()?;
            self.consume(
                TokenType::RightParenthesis,
                "Expected ')' after expression.",
            )?;
            return Ok(Expression::Grouping(Box::new(expression)));
        }

        let token = self.peek().clone();
        Err(ParseError::new(
            format!("Expected expression, found {:?}", token.token_type),
            token.line_number,
        ))
    }

    // ---- helpers ----

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), ParseError> {
        if self.check(&token_type) {
            self.advance();
            return Ok(());
        }

        let token = self.peek().clone();
        Err(ParseError::new(
            format!("{} Found {:?}.", message, token.token_type),
            token.line_number,
        ))
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current_index += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EndOfFile
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_index]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current_index - 1]
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_at_end() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        if self.matches(&[TokenType::Print]) {
            return self.parse_print_statement();
        }

        self.parse_expression_statement()
    }

    fn parse_print_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Statement::Print(expression))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(Statement::Expression(expression))
    }
}
