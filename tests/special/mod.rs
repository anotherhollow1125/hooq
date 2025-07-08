mod fn_special;
mod skip;

// ほかのテストでは不十分と考えられるケースをテスト

#[test]
fn test_walk_special() {
    macrotest::expand("tests/special/fn_special.rs");
    macrotest::expand("tests/special/skip.rs");
}
