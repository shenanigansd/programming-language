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
fn an_undefined_variable_renders_a_caret() {
    let output = run_source("missing;");
    assert!(output.contains('^'));
    assert!(output.contains("undefined variable"));
}

#[test]
fn a_type_mismatch_renders_a_caret_at_the_operator() {
    // 1 + "a"  — the '+' sits at column 2, so the caret should land there, the
    // same visual format as a lex or parse error.
    let output = run_source("1 + \"a\";");
    let caret_line = output
        .lines()
        .nth(1)
        .expect("a caret line below the source");
    assert_eq!(caret_line.find('^'), Some(2));
}

#[test]
fn division_by_zero_renders_a_caret() {
    let output = run_source("5 / 0;");
    assert!(output.contains('^'));
}

#[test]
fn a_parse_error_renders_a_caret() {
    assert!(run_source("let x = 5").contains('^')); // missing semicolon
}

#[test]
fn a_lex_error_renders_a_caret() {
    assert!(run_source("@;").contains('^'));
}
