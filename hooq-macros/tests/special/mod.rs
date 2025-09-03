mod fn_special;
mod hook_targets;
mod r#macro;
mod meta_var;
mod nested;
mod skip;
mod skip_detail;
mod trait_use;

// ほかのテストでは不十分と考えられるケースをテスト

#[test]
fn test_special() {
    crate::mask_project_root("tests/special", crate::UnMask);

    macrotest::expand_args("tests/special/fn_special.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/hook_targets.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/macro.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/nested.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/skip.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/skip_detail.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/trait_use.rs", &["--ugly"]);

    crate::mask_project_root("tests/special", crate::Mask);
}
