use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn snapshot_test_no_unwrap() {
    mask_project_root(".", UnMask);

    let pre_expanded = std::fs::read_to_string("./src/main.expanded.rs").ok();

    let output = String::from_utf8_lossy(
        &std::process::Command::new("cargo")
            .arg("expand")
            .output()
            .unwrap()
            .stdout,
    )
    .to_string();

    if let Some(pre_expanded) = pre_expanded
        && pre_expanded != output
    {
        panic!(
            "snapshot test failed: \n\n--- pre expanded ---\n{pre_expanded}\n\n--- new expanded ---\n{output}\n"
        );
    } else {
        std::fs::write("./src/main.expanded.rs", output).unwrap();
    }

    mask_project_root(".", Mask);
}

#[test]
fn snapshot_test_with_unwrap() {
    mask_project_root(".", UnMask);

    let pre_expanded = std::fs::read_to_string("./src/main.unwrap.expanded.rs").ok();

    let output = String::from_utf8_lossy(
        &std::process::Command::new("cargo")
            .args(["expand", "--features", "unwrap"])
            .output()
            .unwrap()
            .stdout,
    )
    .to_string();

    if let Some(pre_expanded) = pre_expanded
        && pre_expanded != output
    {
        panic!(
            "snapshot test failed: \n\n--- pre expanded ---\n{pre_expanded}\n\n--- new expanded ---\n{output}\n"
        );
    } else {
        std::fs::write("./src/main.unwrap.expanded.rs", output).unwrap();
    }

    mask_project_root(".", Mask);
}
