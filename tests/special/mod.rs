mod abs_path;
mod fn_special;
mod nested;
mod skip;
mod skip_detail;

// ほかのテストでは不十分と考えられるケースをテスト

#[test]
fn test_special() {
    crate::mask_project_root("tests/special", crate::UnMask);

    macrotest::expand_args("tests/special/abs_path.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/fn_special.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/nested.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/skip.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/skip_detail.rs", &["--ugly"]);

    crate::mask_project_root("tests/special", crate::Mask);
}
