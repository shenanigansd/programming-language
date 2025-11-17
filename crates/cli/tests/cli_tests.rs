use std::process::Command;

#[test]
fn print_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_cli"))
        .arg("version")
        .output()
        .expect("Could not run cli command line interface.");

    assert!(output.status.success());

    let text = String::from_utf8(output.stdout).expect("Invalid output encoding.");
    assert!(text.contains("wolf version 0.1.0"));
}
