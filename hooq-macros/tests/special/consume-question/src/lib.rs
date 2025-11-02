pub mod consume_question;

#[cfg(test)]
mod tests {
    use test_helpers::MaskMode::*;
    use test_helpers::mask_project_root;

    use super::consume_question;

    #[test]
    fn test() {
        consume_question::func_unwrap();
        consume_question::func_match().unwrap();
    }

    #[test]
    fn it_works() {
        mask_project_root("src", UnMask);

        macrotest::expand_args("src/consume_question.rs", &["--ugly"]);

        mask_project_root("src", Mask);
    }
}
