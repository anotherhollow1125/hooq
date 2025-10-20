mod method_change {
    use std::fmt::Debug;
    use hooq::hooq;
    fn enresult<T: Debug>(val: T) -> Result<T, ()> {
        Ok(val)
            .inspect(|_| {
                ::std::io::_print(format_args!("ok handler called\n"));
            })
    }
    fn func() -> Result<(), ()> {
        enresult(())
            .inspect_err(|e| {
                let path = "hooq-macros/tests/special/flavor/method-change/tests/tests_inner/method_change.rs";
                let line = 12usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?;
        if enresult(false)
            .inspect(|_| {
                ::std::io::_print(format_args!("ok handler called\n"));
            })?
        {
            return Ok(())
                .inspect(|_| {
                    ::std::io::_print(format_args!("ok handler called\n"));
                });
        }
        if enresult(false)
            .inspect(|_| {
                ::std::io::_print(format_args!("ok handler called\n"));
            })?
        {
            return Ok(());
        }
        enresult(())?;
        Ok(())
    }
    extern crate test;
    #[rustc_test_marker = "tests_inner::method_change::test"]
    #[doc(hidden)]
    pub const test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests_inner::method_change::test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "hooq-macros/tests/special/flavor/method-change/tests/tests_inner/method_change.rs",
            start_line: 33usize,
            start_col: 4usize,
            end_line: 33usize,
            end_col: 8usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test())),
    };
    fn test() {
        func().unwrap();
    }
}
