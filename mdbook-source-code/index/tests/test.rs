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

#[test]
fn expand_snapshot_test() {
    let output = std::process::Command::new("cargo")
        .arg("expand")
        .output()
        .expect("Failed to execute command");

    let re_eprint_to_eprintln = regex::Regex::new(
        r#"\{\s*\n?\s*::std::io::_eprint\(\s*\n?\s*format_args!\((.*?)\),\s*\n?\s*\);\s*\n?\s*\};"#,
    )
    .unwrap();

    let re_tail_expr = regex::Regex::new(r#"Ok\(\s*\n?\s*::alloc::__export::must_use\(\{\s*\n?\s*::alloc::fmt::format\(format_args!\((.*?)\)\)\s*\n?\s*\}\),\s*\n?\s*\)"#).unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout = re_eprint_to_eprintln.replace_all(&stdout, "::std::eprintln!($1);");
    let stdout = re_tail_expr.replace_all(&stdout, "Ok(format!($1))");

    insta::assert_snapshot!("index_expand", format!("{stdout}"));
}
