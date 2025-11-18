mod flavor;
mod flavor_not_cargo_project;
mod fn_special;
mod inert_attr_setting;
mod r#macro;
mod meta_var;
mod nested;
mod skip;
mod skip_detail;
mod source_excerpt_macros;

// ほかのテストでは不十分と考えられるケースをテスト

use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn test_special() {
    mask_project_root("tests/special", UnMask);

    macrotest::expand_args("tests/special/fn_special.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/macro.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/nested.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/skip_detail.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/skip.rs", &["--ugly"]);

    mask_project_root("tests/special", Mask);
}
