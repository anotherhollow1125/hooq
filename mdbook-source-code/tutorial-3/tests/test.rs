#[test]
fn snapshot_test() {
    let output = std::process::Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!(
        "tutorial-3",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}

#[test]
fn snapshot_test_err() {
    let output = std::process::Command::new("cargo")
        .args(["run", "-q", "--", "not_exist.toml"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!(
        "tutorial-3-not-exist",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}

#[test]
fn snapshot_test_err_2() {
    let output = std::process::Command::new("cargo")
        .args(["run", "-q", "--", "test.toml", "2>&1"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!("tutorial-3-name-missing", format!("{}{}", stdout, stderr));
}
