use crate::token::{Token, TokenType};

pub struct Lexer {
    source_characters: Vec<char>,
    start_index: usize,
    current_index: usize,
    current_line_number: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source_characters: source.chars().collect(),
            start_index: 0,
            current_index: 0,
            current_line_number: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start_index = self.current_index;
            self.scan_single_token();
        }

        self.tokens.push(Token::new(
            TokenType::EndOfFile,
            "",
            self.current_line_number,
        ));

        self.tokens
    }

    fn scan_single_token(&mut self) {
        let character = self.advance();

        match character {
            '(' => self.add_token(TokenType::LeftParenthesis),
            ')' => self.add_token(TokenType::RightParenthesis),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                if self.match_character('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }

            '=' => {
                if self.match_character('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }

            '<' => {
                if self.match_character('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }

            '>' => {
                if self.match_character('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            '/' => {
                if self.match_character('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {}

            '\n' => {
                self.current_line_number += 1;
            }

            '"' => {
                self.scan_string_literal();
            }

            character if character.is_ascii_digit() => {
                self.scan_number_literal();
            }

            character if Self::is_alphabetic(character) => {
                self.scan_identifier();
            }

            unexpected_character => {
                eprintln!(
                    "[line {}] Unexpected character: {}",
                    self.current_line_number, unexpected_character
                );
            }
        }
    }

    fn scan_identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let lexeme: String = self.source_characters[self.start_index..self.current_index]
            .iter()
            .collect();

        let token_type = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.tokens
            .push(Token::new(token_type, lexeme, self.current_line_number));
    }

    fn scan_number_literal(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(TokenType::Number);
    }

    fn scan_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.current_line_number += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Unterminated string.", self.current_line_number);
            return;
        }

        self.advance();

        self.add_token(TokenType::String);
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme: String = self.source_characters[self.start_index..self.current_index]
            .iter()
            .collect();

        self.tokens
            .push(Token::new(token_type, lexeme, self.current_line_number));
    }

    fn advance(&mut self) -> char {
        let character = self.source_characters[self.current_index];
        self.current_index += 1;
        character
    }

    fn match_character(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source_characters[self.current_index] != expected {
            return false;
        }

        self.current_index += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_characters[self.current_index]
        }
    }

    fn peek_next(&self) -> char {
        if self.current_index + 1 >= self.source_characters.len() {
            '\0'
        } else {
            self.source_characters[self.current_index + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.source_characters.len()
    }

    fn is_alphabetic(character: char) -> bool {
        character.is_ascii_alphabetic() || character == '_'
    }

    fn is_alphanumeric(character: char) -> bool {
        Self::is_alphabetic(character) || character.is_ascii_digit()
    }
}
