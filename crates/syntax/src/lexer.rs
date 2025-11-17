use crate::token::{Token, TokenKind};

pub struct Lexer {
    characters: Vec<char>,
    position: usize,

    line_number: usize,
    column_number: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            characters: source.chars().collect(),
            position: 0,
            line_number: 1,
            column_number: 1,
        }
    }

    fn current(&self) -> Option<char> {
        self.characters.get(self.position).copied()
    }

    fn advance(&mut self) {
        if let Some(character) = self.current() {
            if character == '\n' {
                self.line_number += 1;
                self.column_number = 1;
            } else {
                self.column_number += 1;
            }
        }

        self.position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let current_character = match self.current() {
            Some(character) => character,
            None => {
                return Token {
                    kind: TokenKind::EndOfFile,
                    text: String::new(),
                    line_number: self.line_number,
                    column_number: self.column_number,
                };
            }
        };

        let token_line = self.line_number;
        let token_column = self.column_number;

        // dispatch based on character
        if current_character.is_ascii_alphabetic() || current_character == '_' {
            return self.lex_identifier(token_line, token_column);
        }

        if current_character.is_ascii_digit() {
            return self.lex_number(token_line, token_column);
        }

        // single-character tokens
        return self.lex_symbol(current_character, token_line, token_column);
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current() {
                Some(' ') | Some('\t') | Some('\r') => {
                    self.advance();
                }
                Some('\n') => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn lex_identifier(&mut self, line: usize, column: usize) -> Token {
        let start_position = self.position;

        // Consume letters, digits, underscores
        while let Some(character) = self.current() {
            if character.is_ascii_alphanumeric() || character == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text: String = self.characters[start_position..self.position]
            .iter()
            .collect();

        let kind = match text.as_str() {
            "let" => TokenKind::Let,
            _ => TokenKind::Identifier,
        };

        Token {
            kind: kind,
            text,
            line_number: line,
            column_number: column,
        }
    }

    fn lex_number(&mut self, line: usize, column: usize) -> Token {
        let start_position = self.position;

        while let Some(character) = self.current() {
            if character.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let text: String = self.characters[start_position..self.position]
            .iter()
            .collect();

        Token {
            kind: TokenKind::Number,
            text,
            line_number: line,
            column_number: column,
        }
    }

    fn lex_symbol(&mut self, character: char, line: usize, column: usize) -> Token {
        self.advance();

        let kind = match character {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '=' => TokenKind::Equal,
            ';' => TokenKind::Semicolon,
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
            _ => {
                // you will eventually have a diagnostics crate,
                // but for now we just panic since you're still learning
                panic!(
                    "Unexpected character '{}' at {}:{}",
                    character, line, column
                );
            }
        };

        Token {
            kind,
            text: character.to_string(),
            line_number: line,
            column_number: column,
        }
    }
}
