pub mod hooq_log;

#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    use super::hooq_log;

    #[test]
    fn test() {
        env_logger::init();

        let _ = hooq_log::c().unwrap_err();
    }

    #[test]
    fn test_special_other_preset_flavors_hooq_log() {
        mask_project_root("src", UnMask);

        macrotest::expand_args("src/hooq_log.rs", &["--ugly"]);

        mask_project_root("src", Mask);
    }
}
