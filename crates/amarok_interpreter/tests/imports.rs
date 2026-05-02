//! Integration tests for the `use` module-loading machinery.

use std::path::PathBuf;

use amarok_interpreter::Interpreter;
use amarok_parser::parse_program;

fn fixtures_dir(subdirectory: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(subdirectory)
}

fn run_with_root(
    source: &str,
    root: PathBuf,
) -> Result<Interpreter, amarok_interpreter::Diagnostic> {
    let program = parse_program(source).expect("Program should parse");
    let mut interpreter = Interpreter::new();
    interpreter.add_module_root(root);
    interpreter.run_program(&program)?;
    Ok(interpreter)
}

#[test]
fn use_imports_function_from_sibling_file() {
    let interpreter = run_with_root("use math; std::print(add(2, 3));", fixtures_dir("single"))
        .expect("import-and-call should succeed");

    assert_eq!(interpreter.output_lines(), &["5".to_string()]);
}

#[test]
fn transitive_imports_resolve() {
    let interpreter = run_with_root(
        "use middle; std::print(double(7));",
        fixtures_dir("transitive"),
    )
    .expect("transitive import should succeed");

    assert_eq!(interpreter.output_lines(), &["14".to_string()]);
}

#[test]
fn diamond_import_evaluates_shared_module_once() {
    let interpreter = run_with_root(
        r#"
            use left;
            use right;
            std::print(left());
            std::print(right());
        "#,
        fixtures_dir("diamond"),
    )
    .expect("diamond import should succeed");

    // "loading shared" is printed by shared.amarok's top-level statement.
    // It must appear exactly once even though both left and right import it.
    assert_eq!(
        interpreter.output_lines(),
        &[
            "loading shared".to_string(),
            "42".to_string(),
            "42".to_string(),
        ]
    );
}

#[test]
fn cyclic_imports_produce_diagnostic() {
    let program = parse_program("use a;").expect("Program should parse");
    let mut interpreter = Interpreter::new();
    interpreter.add_module_root(fixtures_dir("cycle"));

    let error = interpreter
        .run_program(&program)
        .expect_err("cycle should be reported as a diagnostic");

    assert!(
        error.message.contains("Circular import"),
        "expected circular-import diagnostic, got: {}",
        error.message
    );
}

#[test]
fn missing_module_lists_tried_paths() {
    let program = parse_program("use does_not_exist;").expect("Program should parse");
    let mut interpreter = Interpreter::new();
    interpreter.add_module_root(fixtures_dir("single"));

    let error = interpreter
        .run_program(&program)
        .expect_err("missing module should error");

    assert!(
        error.message.contains("Module not found"),
        "expected module-not-found diagnostic, got: {}",
        error.message
    );
    assert!(
        error.message.contains("does_not_exist.amarok"),
        "diagnostic should mention the tried path, got: {}",
        error.message
    );
}

#[test]
fn imported_top_level_variables_are_visible() {
    let interpreter = run_with_root(
        "use greet; std::print(greeting); std::print(get_greeting());",
        fixtures_dir("vars"),
    )
    .expect("import should succeed");

    assert_eq!(
        interpreter.output_lines(),
        &["hi".to_string(), "hi".to_string()]
    );
}

#[test]
fn use_without_module_root_errors() {
    let program = parse_program("use anything;").expect("Program should parse");
    let mut interpreter = Interpreter::new();
    // No add_module_root call.

    let error = interpreter
        .run_program(&program)
        .expect_err("use without roots should fail");

    assert!(error.message.contains("Module not found"));
}
