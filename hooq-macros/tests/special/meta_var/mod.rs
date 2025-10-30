mod bindings;
mod hooq_meta;
mod line;
mod path;

use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn test_special_meta_var() {
    mask_project_root("tests/special/meta_var", UnMask);

    macrotest::expand_args("tests/special/meta_var/path.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/bindings.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/hooq_meta.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/line.rs", &["--ugly"]);

    mask_project_root("tests/special/meta_var", Mask);
}
