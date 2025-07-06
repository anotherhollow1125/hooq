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
    macrotest::expand("tests/walker/expr/await.rs");
    macrotest::expand("tests/walker/expr/binary.rs");
    macrotest::expand("tests/walker/expr/block.rs");
    macrotest::expand("tests/walker/expr/break.rs");
    macrotest::expand("tests/walker/expr/call.rs");
    macrotest::expand("tests/walker/expr/cast.rs");
    macrotest::expand("tests/walker/expr/closure.rs");
    macrotest::expand("tests/walker/expr/const.rs");
    macrotest::expand("tests/walker/expr/field.rs");
    macrotest::expand("tests/walker/expr/for_loop.rs");
    macrotest::expand("tests/walker/expr/group.rs");
    macrotest::expand("tests/walker/expr/if.rs");
    macrotest::expand("tests/walker/expr/index.rs");
    macrotest::expand("tests/walker/expr/let.rs");
    macrotest::expand("tests/walker/expr/loop.rs");
    macrotest::expand("tests/walker/expr/match.rs");
    macrotest::expand("tests/walker/expr/method_call.rs");
    macrotest::expand("tests/walker/expr/paren.rs");
    macrotest::expand("tests/walker/expr/range.rs");
    macrotest::expand("tests/walker/expr/repeat.rs");
    macrotest::expand("tests/walker/expr/struct.rs");
    macrotest::expand("tests/walker/expr/tuple.rs");
    macrotest::expand("tests/walker/expr/unary.rs");
    macrotest::expand("tests/walker/expr/unsafe.rs");
    macrotest::expand("tests/walker/expr/while.rs");
}

#[test]
fn test_walk_special() {
    macrotest::expand("tests/walker/special/fn_special.rs");
}
