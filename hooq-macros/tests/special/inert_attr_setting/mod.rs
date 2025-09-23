mod hook_targets;
mod ignore_tail_expr_idents;
mod option;
mod result_types;
mod tail_expr_idents;
mod trait_use;

use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn test_special_inert_attr_setting() {
    mask_project_root("tests/special/inert_attr_setting", UnMask);

    // マスクを外すファイルの更新が完了する前にテストが走ってしまっているようなので、
    // 少しだけスリープを入れた
    std::thread::sleep(std::time::Duration::from_millis(100));

    macrotest::expand_args(
        "tests/special/inert_attr_setting/hook_targets.rs",
        &["--ugly"],
    );
    macrotest::expand_args(
        "tests/special/inert_attr_setting/ignore_tail_expr_idents.rs",
        &["--ugly"],
    );
    macrotest::expand_args("tests/special/inert_attr_setting/option.rs", &["--ugly"]);
    macrotest::expand_args(
        "tests/special/inert_attr_setting/result_types.rs",
        &["--ugly"],
    );
    macrotest::expand_args(
        "tests/special/inert_attr_setting/tail_expr_idents.rs",
        &["--ugly"],
    );
    macrotest::expand_args("tests/special/inert_attr_setting/trait_use.rs", &["--ugly"]);

    mask_project_root("tests/special/inert_attr_setting", Mask);
}
