use amarok_cli::run_source;

#[test]
fn evaluates_an_expression_statement_with_precedence() {
    assert_eq!(run_source("1 + 2 * 3;"), "7");
}

#[test]
fn a_let_binding_is_usable_by_a_later_statement() {
    assert_eq!(run_source("let x = 5; x + 1;"), "6");
}

#[test]
fn each_expression_statement_prints_on_its_own_line() {
    assert_eq!(run_source("1; 2; 3;"), "1\n2\n3");
}

#[test]
fn a_let_declaration_alone_produces_no_output() {
    assert_eq!(run_source("let x = 5;"), "");
}

#[test]
fn an_undefined_variable_is_a_runtime_error() {
    assert!(run_source("y;").contains("runtime error"));
}

#[test]
fn a_parse_error_renders_a_caret() {
    assert!(run_source("let x = 5").contains("^")); // missing semicolon
}

#[test]
fn a_lex_error_renders_a_caret() {
    assert!(run_source("@;").contains("^"));
}
