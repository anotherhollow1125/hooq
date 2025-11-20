#[test]
fn snapshot_test_without_envs() {
    let output = std::process::Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!(
        "index_without_envs",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}

#[test]
fn snapshot_test_with_envs() {
    let output = std::process::Command::new("cargo")
        .env("APP_HOST", "127.0.0.1")
        .env("APP_PORT", "10")
        .args(["run", "-q"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!(
        "index_with_envs",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}

#[test]
fn snapshot_test_with_app_port() {
    let output = std::process::Command::new("cargo")
        .env("APP_PORT", "10")
        .args(["run", "-q"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!(
        "index_with_app_port",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}
