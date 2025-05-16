#[test]
fn test_walk_stmt() {
    macrotest::expand("tests/walker/stmt/local.rs");
    macrotest::expand("tests/walker/stmt/item.rs");
    macrotest::expand("tests/walker/stmt/expr.rs");
}

#[test]
fn test_walk_item() {
    macrotest::expand("tests/walker/item/fn.rs");
}
