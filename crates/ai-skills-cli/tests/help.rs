use std::process::Command;

#[test]
fn help_is_available_from_the_workspace_binary() {
    let output = Command::new(env!("CARGO_BIN_EXE_ai-skills"))
        .arg("--help")
        .output()
        .expect("the ai-skills binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("help output should be UTF-8");
    assert!(stdout.contains("Usage:"));
}
