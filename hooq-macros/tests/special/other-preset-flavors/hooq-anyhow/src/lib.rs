pub mod hooq_anyhow;

#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    use super::hooq_anyhow;

    #[test]
    #[should_panic]
    fn test() {
        hooq_anyhow::c().unwrap();
    }

    #[test]
    fn test_special_other_preset_flavors_hooq_anyhow() {
        mask_project_root("src", UnMask);

        macrotest::expand_args("src/hooq_anyhow.rs", &["--ugly"]);

        mask_project_root("src", Mask);
    }
}
