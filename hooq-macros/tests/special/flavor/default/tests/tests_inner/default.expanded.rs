mod default {
    use std::fmt::Debug;
    use hooq::hooq;
    #[allow(unused)]
    fn enresult<T: Debug>(val: T) -> Result<T, ()> {
        Ok(val)
    }
    #[allow(unused)]
    fn func(flag: bool) -> Result<(), ()> {
        enresult(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[default]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[default]",
                        e,
                    ),
                );
            })?;
        if flag {
            return Err(())
                .inspect(|v| {
                    ::std::io::_print(
                        format_args!(
                            "Ok Value with: {1:?} & with tag: {0}\n",
                            "[default]",
                            v,
                        ),
                    );
                })
                .inspect_err(|e| {
                    ::std::io::_eprint(
                        format_args!(
                            "Err Value with: {1:?} & with tag: {0}\n",
                            "[default]",
                            e,
                        ),
                    );
                });
        }
        Err(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[default]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[default]",
                        e,
                    ),
                );
            })
    }
    extern crate test;
    #[rustc_test_marker = "tests_inner::default::test"]
    #[doc(hidden)]
    pub const test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests_inner::default::test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "hooq-macros/tests/special/flavor/default/tests/tests_inner/default.rs",
            start_line: 24usize,
            start_col: 4usize,
            end_line: 24usize,
            end_col: 8usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test())),
    };
    fn test() {
        func(false).unwrap_err();
    }
}
