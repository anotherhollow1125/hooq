mod custom {
    use std::fmt::Debug;
    use hooq::hooq;
    #[allow(unused)]
    fn enresult<T: Debug>(val: T) -> Result<T, ()> {
        Ok(val)
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
    #[allow(unused)]
    fn func(flag: bool) -> Result<(), ()> {
        enresult(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[sub]", v),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!("Err Value with: {1:?} & with tag: {0}\n", "[sub]", e),
                );
            })?;
        if flag {
            return Ok(())
                .inspect(|v| {
                    ::std::io::_print(
                        format_args!(
                            "Ok Value with: {1:?} & with tag: {0}\n",
                            "[sub]",
                            v,
                        ),
                    );
                })
                .inspect_err(|e| {
                    ::std::io::_eprint(
                        format_args!(
                            "Err Value with: {1:?} & with tag: {0}\n",
                            "[sub]",
                            e,
                        ),
                    );
                });
        }
        let _ = { Result::<(), ()>::Ok(()) };
        Err(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[sub]", v),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!("Err Value with: {1:?} & with tag: {0}\n", "[sub]", e),
                );
            })
    }
    #[allow(unused)]
    fn func2(flag: bool) -> Result<(), ()> {
        enresult(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[sub.sub]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[sub.sub]",
                        e,
                    ),
                );
            })?;
        if flag {
            return Ok(())
                .inspect(|v| {
                    ::std::io::_print(
                        format_args!(
                            "Ok Value with: {1:?} & with tag: {0}\n",
                            "[sub.sub]",
                            v,
                        ),
                    );
                })
                .inspect_err(|e| {
                    ::std::io::_eprint(
                        format_args!(
                            "Err Value with: {1:?} & with tag: {0}\n",
                            "[sub.sub]",
                            e,
                        ),
                    );
                });
        }
        let res = {
            Ok(())
                .inspect(|v| {
                    ::std::io::_print(
                        format_args!(
                            "Ok Value with: {1:?} & with tag: {0}\n",
                            "[sub.sub]",
                            v,
                        ),
                    );
                })
                .inspect_err(|e| {
                    ::std::io::_eprint(
                        format_args!(
                            "Err Value with: {1:?} & with tag: {0}\n",
                            "[sub.sub]",
                            e,
                        ),
                    );
                })
        };
        {
            ::std::io::_print(format_args!("res: {0:?}\n", res));
        };
        res.inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[sub.sub]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[sub.sub]",
                        e,
                    ),
                );
            })
    }
    #[allow(unused)]
    fn func3(flag: bool) -> Result<(), ()> {
        enresult(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_1]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_1]",
                        e,
                    ),
                );
            })?;
        if flag {
            return Ok(());
        }
        let res = { Ok(()) };
        {
            ::std::io::_print(format_args!("res: {0:?}\n", res));
        };
        res.inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_1]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_1]",
                        e,
                    ),
                );
            })
    }
    #[allow(unused)]
    fn func4(flag: bool) -> Result<(), ()> {
        enresult(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_2]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_2]",
                        e,
                    ),
                );
            })?;
        if flag {
            return Ok(());
        }
        let res = { Ok(()) };
        {
            ::std::io::_print(format_args!("res: {0:?}\n", res));
        };
        res.inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_2]",
                        v,
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n",
                        "[ignore_tail_expr_idents_test_2]",
                        e,
                    ),
                );
            })
    }
    extern crate test;
    #[rustc_test_marker = "tests_inner::custom::test"]
    #[doc(hidden)]
    pub const test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests_inner::custom::test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "hooq-macros/tests/special/flavor/custom/tests/tests_inner/custom.rs",
            start_line: 74usize,
            start_col: 4usize,
            end_line: 74usize,
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
        func(true).unwrap();
        func2(true).unwrap();
        func2(false).unwrap();
        func3(true).unwrap();
        func3(false).unwrap();
        func4(true).unwrap();
        func4(false).unwrap();
    }
}
