use std::thread::sleep;
use std::time::Duration;

#[test]
fn snapshot_test() {
    prepare();

    sleep(Duration::from_secs(1));

    let output_w = std::process::Command::new("cargo")
        .args(["run", "-q"])
        .env("CARGO_TERM_COLOR", "never")
        .output();

    sleep(Duration::from_secs(1));

    cleanup();

    sleep(Duration::from_secs(1));

    let output = output_w.expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!(
        "tutorial-4-compile-error",
        format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    );
}

fn prepare() {
    let target_main_content = std::fs::read_to_string("src/_main.txt").unwrap();

    std::fs::write("src/main.rs", target_main_content).unwrap();
}

fn cleanup() {
    let hello_world = r#"fn main() {
    println!("Hello, world!");
}
"#;

    std::fs::write("src/main.rs", hello_world).unwrap();
}
