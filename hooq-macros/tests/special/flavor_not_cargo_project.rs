#[test]
fn test_flavor_not_cargo_project() {
    let res = std::process::Command::new("./build_script.rs")
        .current_dir("tests/special/flavor_not_cargo_project")
        .status()
        .expect("Failed to execute build_script");

    assert!(res.success());
}
