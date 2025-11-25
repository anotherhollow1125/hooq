use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn snapshot_test() {
    mask_project_root(".", UnMask);

    macrotest::expand_args("./src/main.rs", &["--ugly"]);

    mask_project_root(".", Mask);
}
