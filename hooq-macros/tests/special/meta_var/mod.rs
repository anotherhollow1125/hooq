mod bindings;
mod fn_info;
mod hooq_meta;
mod line;
mod path;
mod so_far;
mod source;

use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn test_special_meta_var() {
    mask_project_root("tests/special/meta_var", UnMask);

    macrotest::expand_args("tests/special/meta_var/bindings.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/source.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/fn_info.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/hooq_meta.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/line.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/path.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/so_far.rs", &["--ugly"]);

    mask_project_root("tests/special/meta_var", Mask);
}
