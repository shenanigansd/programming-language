use amarok_diagnostics::{Diagnostic, SourcePosition, render};

#[test]
fn caret_points_at_the_offending_character() {
    let diagnostic = Diagnostic::new(
        "unexpected character: '@'",
        SourcePosition { character_index: 2 },
    );
    assert_eq!(
        render("1 @ 2", &diagnostic),
        "1 @ 2\n  ^ unexpected character: '@'",
    );
}

#[test]
fn caret_at_the_very_first_column() {
    let diagnostic = Diagnostic::new(
        "unexpected character: '@'",
        SourcePosition { character_index: 0 },
    );
    assert_eq!(
        render("@foo", &diagnostic),
        "@foo\n^ unexpected character: '@'"
    );
}

#[test]
fn caret_lands_on_the_correct_line_of_multiline_source() {
    // "abc\nde@f" — '@' is index 6, on the second line, column 2 of that line.
    let diagnostic = Diagnostic::new(
        "unexpected character: '@'",
        SourcePosition { character_index: 6 },
    );
    assert_eq!(
        render("abc\nde@f", &diagnostic),
        "de@f\n  ^ unexpected character: '@'"
    );
}
