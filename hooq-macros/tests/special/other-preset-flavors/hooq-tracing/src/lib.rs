pub mod hooq_tracing;

#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    use super::hooq_tracing;

    fn test_inner() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();

        hooq_tracing::c()?;

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test() {
        test_inner().unwrap();
    }

    #[test]
    fn test_special_other_preset_flavors_hooq_tracing() {
        mask_project_root("src", UnMask);

        macrotest::expand_args("src/hooq_tracing.rs", &["--ugly"]);

        mask_project_root("src", Mask);
    }
}
