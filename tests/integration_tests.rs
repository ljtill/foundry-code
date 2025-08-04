use std::process::Command;

#[test]
fn test_binary_exists() {
    let output = Command::new("cargo")
        .args(["build", "--bin", "foundry"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo build");

    assert!(output.status.success(), "Failed to build foundry binary");
}

#[test]
fn test_help_flag() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "foundry", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute foundry --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("foundry") || stdout.contains("Usage"));
}

#[test]
fn test_version_flag() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "foundry", "--", "--version"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute foundry --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());
}

#[test]
fn test_no_args_does_not_crash_immediately() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "foundry"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .spawn()
        .expect("Failed to start foundry without arguments");

    std::thread::sleep(std::time::Duration::from_millis(100));

    let status = child.try_wait().expect("Failed to check process status");
    assert!(status.is_none(), "TUI process exited unexpectedly");

    let _ = child.kill();
    let _ = child.wait();
}
