#[test]
fn test_walk_stmt() {
    macrotest::expand("tests/walker/stmt/local.rs");
    macrotest::expand("tests/walker/stmt/item.rs");
    macrotest::expand("tests/walker/stmt/expr.rs");
}

#[test]
fn test_walk_item() {
    macrotest::expand("tests/walker/item/fn.rs");
    macrotest::expand("tests/walker/item/impl.rs");
    macrotest::expand("tests/walker/item/mod_.rs");
}

#[test]
fn test_walk_expr() {
    macrotest::expand("tests/walker/expr/try.rs");
    macrotest::expand("tests/walker/expr/return.rs");
    macrotest::expand("tests/walker/expr/array.rs");
    macrotest::expand("tests/walker/expr/assign.rs");
    macrotest::expand("tests/walker/expr/async.rs");
}

#[test]
fn test_walk_special() {
    macrotest::expand("tests/walker/special/fn_special.rs");
}
