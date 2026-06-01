use amarok_cli::run_line;

#[test]
fn evaluates_arithmetic_with_precedence() {
    assert_eq!(run_line("1 + 2 * 3"), "7");
}

#[test]
fn parentheses_override_precedence() {
    assert_eq!(run_line("(1 + 2) * 3"), "9");
}

#[test]
fn evaluates_a_comparison_to_a_boolean() {
    assert_eq!(run_line("1 < 2"), "true");
}

#[test]
fn concatenates_strings() {
    assert_eq!(run_line("\"foo\" + \"bar\""), "foobar");
}

#[test]
fn a_blank_line_produces_no_output() {
    assert_eq!(run_line("   "), "");
}

#[test]
fn a_lex_error_renders_a_caret() {
    let output = run_line("1 @ 2");
    assert!(output.contains("1 @ 2"));
    assert!(output.contains("^"));
    assert!(output.contains("unexpected character"));
}

#[test]
fn reports_a_parse_error() {
    assert!(run_line("(1 + 2").contains("parse error"));
}

#[test]
fn reports_a_runtime_error() {
    assert!(run_line("1 / 0").contains("runtime error"));
}
