use crate::ast::*;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or_else(|| {
            self.tokens
                .last()
                .expect("Parser must have at least one token")
        })
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.current().kind == kind {
            self.advance();
            return true;
        }
        false
    }

    // Entry point for expressions
    pub fn parse_expression(&mut self) -> ExpressionNode {
        self.parse_term()
    }

    fn parse_term(&mut self) -> ExpressionNode {
        let mut left = self.parse_factor();

        loop {
            let operator = match self.current().kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.advance();

            let right = self.parse_factor();

            left = ExpressionNode::BinaryOperation {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_factor(&mut self) -> ExpressionNode {
        let mut left = self.parse_primary();

        loop {
            let operator = match self.current().kind {
                TokenKind::Star => BinaryOperator::Multiply,
                TokenKind::Slash => BinaryOperator::Divide,
                _ => break,
            };

            self.advance();

            let right = self.parse_primary();

            left = ExpressionNode::BinaryOperation {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary(&mut self) -> ExpressionNode {
        let token = self.current().clone();

        match token.kind {
            TokenKind::Number => {
                self.advance();
                let value: i64 = token.text.parse().unwrap();
                ExpressionNode::NumberLiteral { value }
            }

            TokenKind::Identifier => {
                self.advance();
                ExpressionNode::IdentifierReference {
                    name: token.text.clone(),
                }
            }

            TokenKind::LeftParenthesis => {
                self.advance();
                let expression = self.parse_expression();
                if !self.match_kind(TokenKind::RightParenthesis) {
                    panic!("Expected closing parenthesis");
                }
                expression
            }

            _ => panic!(
                "Unexpected token {:?} at {}:{}",
                token.kind, token.line_number, token.column_number
            ),
        }
    }

    fn parse_statement(&mut self) -> StatementNode {
        match self.current().kind {
            TokenKind::Let => self.parse_variable_declaration(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_variable_declaration(&mut self) -> StatementNode {
        // consume 'let'
        self.advance();

        // must be an identifier next
        let name_token = self.current().clone();
        if name_token.kind != TokenKind::Identifier {
            panic!(
                "Expected identifier after 'let' at {}:{}",
                name_token.line_number, name_token.column_number
            );
        }
        self.advance();
        let name = name_token.text;

        // must have '='
        if self.current().kind != TokenKind::Equal {
            panic!(
                "Expected '=' after variable name '{}' at {}:{}",
                name, name_token.line_number, name_token.column_number
            );
        }
        self.advance();

        // parse initializer
        let value = self.parse_expression();

        // semicolon ends the declaration
        if !self.match_kind(TokenKind::Semicolon) {
            let token = self.current();
            panic!(
                "Expected semicolon after variable declaration at {}:{}",
                token.line_number, token.column_number
            );
        }

        StatementNode::VariableDeclaration { name, value }
    }

    fn parse_expression_statement(&mut self) -> StatementNode {
        let expression = self.parse_expression();

        if !self.match_kind(TokenKind::Semicolon) {
            let token = self.current();
            panic!(
                "Expected semicolon at {}:{}, found {:?}",
                token.line_number, token.column_number, token.kind
            );
        }

        StatementNode::ExpressionStatement { expression }
    }

    pub fn parse_program(&mut self) -> ProgramNode {
        let mut statements = Vec::new();

        // parse until EndOfFile
        while self.current().kind != TokenKind::EndOfFile {
            let statement = self.parse_statement();
            statements.push(statement);
        }

        ProgramNode { statements }
    }
}
