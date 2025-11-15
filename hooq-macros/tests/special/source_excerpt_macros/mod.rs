use hooq::source_excerpt_helpers::{
    excerpted_pretty_stringify, one_line_stringify, pretty_stringify,
};

#[test]
fn test_pretty_stringify_without_options() {
    #[rustfmt::skip]
    let res = pretty_stringify!(
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in pretty stringify macro",
        ])
    );

    insta::assert_snapshot!("pretty_stringify_without_options", res);
}

#[test]
fn test_pretty_stringify_with_line_numbers() {
    #[rustfmt::skip]
    let res = pretty_stringify!(@with_line,
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
        ])
    );

    insta::assert_snapshot!("pretty_stringify_with_line_numbers", res);
}

#[test]
fn test_pretty_stringify_with_padding() {
    // instaの性質か、空行だけだとスナップショットからtrimされてしまう模様
    #[rustfmt::skip]
    let res = pretty_stringify!(@padding,
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in pretty stringify macro",
        ])
    );
    insta::assert_snapshot!("pretty_stringify_with_padding", res);
}

#[test]
fn test_pretty_stringify_with_line_numbers_and_padding() {
    #[rustfmt::skip]
    let res = pretty_stringify!(
        @with_line,
        @padding = "both"
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in pretty stringify macro",
        ])
    );

    insta::assert_snapshot!("pretty_stringify_with_line_numbers_and_padding", res);
}

#[test]
fn test_pretty_stringify_with_line_numbers_and_bottom_padding() {
    #[rustfmt::skip]
    let res = pretty_stringify!(
        @with_line,
        @padding = "bottom"
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in pretty stringify macro",
        ])
    );

    insta::assert_snapshot!("pretty_stringify_with_line_numbers_and_bottom_padding", res);
}

#[test]
fn test_one_line_stringify_without_options() {
    #[rustfmt::skip]
    let res = one_line_stringify!(
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in one line stringify macro",
        ])
    );

    insta::assert_snapshot!("one_line_stringify_without_options", res);
}

#[test]
fn test_one_line_stringify_with_truncation() {
    #[rustfmt::skip]
    let res = one_line_stringify!(@truncate_str,
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in one line stringify macro",
        ])
    );

    insta::assert_snapshot!("one_line_stringify_with_truncation", res);
}

#[test]
fn test_one_line_stringify_with_truncation_30() {
    #[rustfmt::skip]
    let res = one_line_stringify!(
        @truncate_str = 30
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in one line stringify macro",
        ])
    );

    insta::assert_snapshot!("one_line_stringify_with_truncation_30", res);
}

#[test]
fn test_excerpted_pretty_stringify_without_options() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in excerpted pretty stringify macro",
        ])
    );

    insta::assert_snapshot!("excerpted_pretty_stringify_without_options", res);
}

#[test]
fn test_excerpted_pretty_stringify_with_full_options() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(
        @excerpt_line = 4,
        @truncate_str,
        @with_line,
        @padding,
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in excerpted pretty stringify macro",
        ])
    );

    insta::assert_snapshot!("excerpted_pretty_stringify_with_full_options", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_return() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(
        return Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            10,
            42,
            "long string to test truncation behavior in excerpted pretty stringify macro",
        ])
    );

    insta::assert_snapshot!("excerpted_pretty_stringify_for_return", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_question_mark() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(
        func_call(|| {
            let s = "hello!";

            println!("{s}");
        })?
    );

    insta::assert_snapshot!("excerpted_pretty_stringify_for_question_mark", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_tailexpr_oneline() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(Ok(()));

    insta::assert_snapshot!("excerpted_pretty_stringify_for_tailexpr_oneline", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_return_oneline() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(return Ok(()));

    insta::assert_snapshot!("excerpted_pretty_stringify_for_return_oneline", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_question_mark_oneline() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(hoge()?);

    insta::assert_snapshot!("excerpted_pretty_stringify_for_question_mark_oneline", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_tailexpr_twolines() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(Ok(
    "aaaa"));

    insta::assert_snapshot!("excerpted_pretty_stringify_for_tailexpr_twolines", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_return_twolines() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(return Ok(
    "aaaa"));

    insta::assert_snapshot!("excerpted_pretty_stringify_for_return_twolines", res);
}

#[test]
fn test_excerpted_pretty_stringify_for_question_mark_twolines() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(hoge()
    .fuga()?);
    insta::assert_snapshot!("excerpted_pretty_stringify_for_question_mark_twolines", res);
}

#[test]
fn test_excerpted_pretty_stringify_long_multilines_30() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(
        @excerpt_line = 30,
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            "eee",
            "fff",
            "ggg",
            "hhh",
            "iii",
            "jjj",
            "kkk",
            "lll",
            "mmm",
        ])
    );

    insta::assert_snapshot!("excerpted_pretty_stringify_long_multilines_30", res);
}

#[test]
fn test_excerpted_pretty_stringify_long_multilines_10() {
    #[rustfmt::skip]
    let res = excerpted_pretty_stringify!(
        @excerpt_line = 10,
        Err([
            "aaa",
            "bbb",
            "ccc",
            "ddd",
            "eee",
            "fff",
            "ggg",
            "hhh",
            "iii",
            "jjj",
            "kkk",
            "lll",
            "mmm",
        ])
    );

    insta::assert_snapshot!("excerpted_pretty_stringify_long_multilines_10", res);
}
