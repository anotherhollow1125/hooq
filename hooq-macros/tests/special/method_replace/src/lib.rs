pub mod method_replace;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    use super::method_replace;

    #[test]
    fn test_method_replace() {
        method_replace::func_unwrap().unwrap();
        method_replace::func_overridden().unwrap();
    }

    #[test]
    fn test_special_method_replace() {
        mask_project_root("src", UnMask);

        macrotest::expand_args("src/method_replace.rs", &["--ugly"]);

        mask_project_root("src", Mask);
    }
}
