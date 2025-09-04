mod abs_path;
mod bindings;
mod hooq_meta;

#[test]
fn test_special_meta_var() {
    crate::mask_project_root("tests/special/meta_var", crate::UnMask);

    macrotest::expand_args("tests/special/meta_var/abs_path.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/bindings.rs", &["--ugly"]);
    macrotest::expand_args("tests/special/meta_var/hooq_meta.rs", &["--ugly"]);

    crate::mask_project_root("tests/special/meta_var", crate::Mask);
}
