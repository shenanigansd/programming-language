pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use ast::ProgramNode;
use lexer::Lexer;
use parser::Parser;

pub fn parse_source(source: &str) -> ProgramNode {
    let mut lexer = Lexer::new(source);

    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        let end = token.kind == token::TokenKind::EndOfFile;
        tokens.push(token);
        if end {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
