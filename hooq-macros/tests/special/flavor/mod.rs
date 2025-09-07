mod empty;

use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn test_special_flavor() {
    mask_project_root("tests/special/flavor", UnMask);

    macrotest::expand_args("tests/special/flavor/empty.rs", &["--ugly"]);

    mask_project_root("tests/special/flavor", Mask);
}
