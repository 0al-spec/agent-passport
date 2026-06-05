use std::process::Command;

#[test]
fn validate_example_passport() {
    let output = Command::new(env!("CARGO_BIN_EXE_agent-passport"))
        .args(["validate", "examples/log-processor.passport.yaml"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run agent-passport CLI");

    assert!(
        output.status.success(),
        "expected successful validation, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK examples/log-processor.passport.yaml"));
}

#[test]
fn validate_example_passport_as_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_agent-passport"))
        .args(["validate", "--json", "examples/log-processor.passport.yaml"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run agent-passport CLI");

    assert!(
        output.status.success(),
        "expected successful validation, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#""valid": true"#));
    assert!(stdout.contains(r#""checks": []"#));
}
