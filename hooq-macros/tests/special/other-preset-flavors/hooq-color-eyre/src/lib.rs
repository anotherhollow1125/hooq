pub mod hooq_color_eyre;

#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    use super::hooq_color_eyre;

    #[test]
    #[should_panic]
    fn test() {
        hooq_color_eyre::c().unwrap();
    }

    #[test]
    fn test_special_other_preset_flavors_hooq_color_eyre() {
        mask_project_root("src", UnMask);

        macrotest::expand_args("src/hooq_color_eyre.rs", &["--ugly"]);

        mask_project_root("src", Mask);
    }
}
