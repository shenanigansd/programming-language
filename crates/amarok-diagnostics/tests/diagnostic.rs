use amarok_diagnostics::{Diagnostic, SourcePosition};

#[test]
fn diagnostic_stores_its_message_and_position() {
    let diagnostic = Diagnostic::new(
        "unexpected character",
        SourcePosition { character_index: 7 },
    );
    assert_eq!(diagnostic.message, "unexpected character");
    assert_eq!(diagnostic.position, SourcePosition { character_index: 7 });
}

#[test]
fn diagnostic_new_accepts_an_owned_string() {
    let owned = String::from("a message built at runtime");
    let diagnostic = Diagnostic::new(owned, SourcePosition { character_index: 0 });
    assert_eq!(diagnostic.message, "a message built at runtime");
}
