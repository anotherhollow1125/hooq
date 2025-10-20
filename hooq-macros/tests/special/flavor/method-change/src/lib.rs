#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    #[test]
    fn test_special_flavor_method_change() {
        mask_project_root("tests/tests_inner", UnMask);

        // macrotest は hooq_toml を読み込んでくれないためここだけ手動でスナップショットテストを行う
        // macrotest::expand_args("tests/tests_inner/method_change.rs", &["--ugly"]);

        let pre_expanded =
            std::fs::read_to_string("tests/tests_inner/method_change.expanded.rs").ok();

        let output = String::from_utf8_lossy(
            &std::process::Command::new("cargo")
                .args(["expand", "--test", "mod", "tests_inner::method_change"])
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
            std::fs::write("tests/tests_inner/method_change.expanded.rs", output).unwrap();
        }

        mask_project_root("tests/tests_inner", Mask);
    }
}
