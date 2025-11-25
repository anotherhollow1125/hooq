#[test]
fn snapshot_test() {
    let output = std::process::Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!(
        "flavor-anyhow",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}
