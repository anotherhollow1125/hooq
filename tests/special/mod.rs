mod fn_special;
mod skip;
mod skip_detail;

// ほかのテストでは不十分と考えられるケースをテスト

#[test]
fn test_walk_special() {
    crate::mask_project_root("tests/special", crate::UnMask);

    macrotest::expand("tests/special/fn_special.rs");
    macrotest::expand("tests/special/skip.rs");
    macrotest::expand("tests/special/skip_detail.rs");

    crate::mask_project_root("tests/special", crate::Mask);
}
