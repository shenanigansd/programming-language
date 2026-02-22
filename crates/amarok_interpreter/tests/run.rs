use amarok_interpreter::Interpreter;
use amarok_parser::parse_program;

#[test]
fn runs_assignment_and_print() {
    let program = parse_program("x = 1 + 2; print(x);").expect("Program should parse");

    let mut interpreter = Interpreter::new();
    interpreter
        .run_program(&program)
        .expect("Program should run");

    assert_eq!(interpreter.output_lines(), &["3".to_string()]);
}

#[test]
fn runs_function_definition_and_call() {
    let source = r#"
        def add(a, b) { return a + b; }
        x = add(2, 3);
        print(x);
    "#;

    let program = parse_program(source).expect("Program should parse");

    let mut interpreter = Interpreter::new();
    interpreter
        .run_program(&program)
        .expect("Program should run");

    assert_eq!(interpreter.output_lines(), &["5".to_string()]);
}

#[test]
fn while_loop_counts_down() {
    let source = r#"
        x = 3;
        while (x) {
            print(x);
            x = x - 1;
        }
    "#;

    let program = parse_program(source).expect("Program should parse");

    let mut interpreter = Interpreter::new();
    interpreter
        .run_program(&program)
        .expect("Program should run");

    assert_eq!(
        interpreter.output_lines(),
        &["3".to_string(), "2".to_string(), "1".to_string()]
    );
}
