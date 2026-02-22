use amarok_lexer::lexer::Lexer;
use amarok_parser::parser::Parser;
use amarok_parser::statement::Statement;

fn parse_program(source: &str) -> Vec<Statement> {
    let tokens = Lexer::new(source).scan_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse_program().unwrap()
}

#[test]
fn parses_print_statement() {
    let program = parse_program("print 123;");

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::Print(_) => {}
        other => panic!("Expected print statement, got {:?}", other),
    }
}

#[test]
fn parses_expression_statement() {
    let program = parse_program("1 + 2;");

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::Expression(_) => {}
        other => panic!("Expected expression statement, got {:?}", other),
    }
}

#[test]
fn parses_multiple_statements() {
    let program = parse_program("print 1; 2 + 3; print 4;");

    assert_eq!(program.len(), 3);
    matches!(&program[0], Statement::Print(_));
    matches!(&program[1], Statement::Expression(_));
    matches!(&program[2], Statement::Print(_));
}
