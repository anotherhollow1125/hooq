#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    #[test]
    fn test_special_flavor_default() {
        mask_project_root("tests/tests_inner", UnMask);

        macrotest::expand_args("tests/tests_inner/default.rs", &["--ugly"]);

        mask_project_root("tests/tests_inner", Mask);
    }
}
