use std::process::Command;

#[test]
fn test_exit_code_config_error() {
    // Test missing database URL (should exit with code 2)
    let output = Command::new("cargo")
        .args([
            "run",
            "--release",
            "--",
            "--query",
            "SELECT 1",
            "--output",
            "/tmp/test.csv",
        ])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Database URL resolution failed"));
}

#[test]
fn test_exit_code_success() {
    // Test help command (should exit with code 0)
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_exit_code_dump_config() {
    // Test dump-config command (should exit with code 0)
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "--dump-config"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("database_url"));
}
