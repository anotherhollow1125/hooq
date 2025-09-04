mod hook_targets;
mod option;
mod result_types;
mod tail_expr_idents;
mod trait_use;

#[test]
fn test_special_inert_attr_setting() {
    crate::mask_project_root("tests/special/inert_attr_setting", crate::UnMask);

    macrotest::expand_args(
        "tests/special/inert_attr_setting/hook_targets.rs",
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

    crate::mask_project_root("tests/special/inert_attr_setting", crate::Mask);
}
