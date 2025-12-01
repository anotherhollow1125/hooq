use regex::Regex;

#[test]
#[ignore]
fn snapshot_test() {
    let output = std::process::Command::new("cargo")
        .env("RUST_LIB_BACKTRACE", "1")
        .args(["run", "-q"])
        .output()
        .expect("Failed to execute command");

    let re = Regex::new(r"/home/.*?/").unwrap();

    let stdout = strip_ansi_escapes::strip(&output.stdout);
    let stderr = strip_ansi_escapes::strip(&output.stderr);

    let stdout = String::from_utf8_lossy(&stdout);
    let stdout = re.replace_all(&stdout, "/home/USER/");
    let stderr = String::from_utf8_lossy(&stderr);
    let stderr = re.replace_all(&stderr, "/home/USER/");

    insta::assert_snapshot!(
        "recipe-color-eyre",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}
