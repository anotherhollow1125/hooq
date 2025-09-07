use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn test_walk_stmt() {
    mask_project_root("tests/walker/stmt", UnMask);

    macrotest::expand_args("tests/walker/stmt/local.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/stmt/item.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/stmt/expr.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/stmt/macro.rs", &["--ugly"]);

    mask_project_root("tests/walker/stmt", Mask);
}

#[test]
fn test_walk_item() {
    mask_project_root("tests/walker/item", UnMask);

    macrotest::expand_args("tests/walker/item/const.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/fn.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/impl.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/macro.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/mod_.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/static.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/trait.rs", &["--ugly"]);

    mask_project_root("tests/walker/item", Mask);
}

#[test]
fn test_walk_expr() {
    mask_project_root("tests/walker/expr", UnMask);

    macrotest::expand_args("tests/walker/expr/try.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/return.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/array.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/assign.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/async.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/await.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/binary.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/block.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/break.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/call.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/cast.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/closure.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/const.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/field.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/for_loop.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/group.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/if.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/index.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/let.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/loop.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/macro.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/match.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/method_call.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/paren.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/range.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/repeat.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/struct.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/tuple.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/unary.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/unsafe.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/while.rs", &["--ugly"]);

    mask_project_root("tests/walker/expr", Mask);
}
