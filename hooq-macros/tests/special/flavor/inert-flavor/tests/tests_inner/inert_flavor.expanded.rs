mod inert_flavor {
    use std::fmt::Debug;
    use hooq::hooq;
    fn enresult<T: Debug>(val: T) -> Result<T, ()> {
        Ok(val)
            .inspect(|_| {
                ::std::io::_print(format_args!("ok handler called\n"));
            })
    }
    type MyResult = Result<(), ()>;
    #[allow(non_snake_case)]
    fn MyOk() -> MyResult {
        Ok(())
    }
    #[allow(non_snake_case)]
    fn MyErr() -> MyResult {
        Err(())
    }
    fn flavor_override() -> Result<(), ()> {
        enresult(())
            .inspect_err(|e| {
                let path = "hooq-macros/tests/special/flavor/inert-flavor/tests/tests_inner/inert_flavor.rs";
                let line = 24usize;
                let col = 17usize;
                let expr = "enresult(())?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n",
                            path,
                            line,
                            col,
                            e,
                            expr,
                        ),
                    );
                };
            })?;
        let f = || -> MyResult {
            enresult(())
                .inspect(|_| {
                    ::std::io::_eprint(
                        format_args!("full custom handler called: {0}\n", "Hello"),
                    );
                })?;
            {
                ::std::io::_print(format_args!("{0}\n", enresult(10)?));
            };
            if enresult(false)
                .inspect(|_| {
                    ::std::io::_eprint(
                        format_args!("full custom handler called: {0}\n", "Hello"),
                    );
                })?
            {
                return MyErr();
            }
            let g = || -> MyResult {
                enresult(())
                    .inspect(|_| {
                        ::std::io::_eprint(
                            format_args!("full custom handler called: {0}\n", "Hello"),
                        );
                    })
            };
            let h = || -> Result<(), ()> { enresult(()) };
            g()
                .inspect(|_| {
                    ::std::io::_eprint(
                        format_args!("full custom handler called: {0}\n", "Hello"),
                    );
                })?;
            h()?;
            let _ = {
                MyErr()
                    .inspect(|_| {
                        ::std::io::_eprint(
                            format_args!("full custom handler called: {0}\n", "Hello"),
                        );
                    })
            };
            MyOk()
        };
        {
            ::std::io::_print(
                format_args!(
                    "{0:?}\n",
                    f()
                        .inspect_err(|e| {
                            let path = "hooq-macros/tests/special/flavor/inert-flavor/tests/tests_inner/inert_flavor.rs";
                            let line = 48usize;
                            let col = 25usize;
                            let expr = "f()?";
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "[{0}:{1}:{2}] {3:?}\n    {4}\n",
                                        path,
                                        line,
                                        col,
                                        e,
                                        expr,
                                    ),
                                );
                            };
                        })?,
                ),
            );
        };
        enresult(())
            .inspect_err(|e| {
                let path = "hooq-macros/tests/special/flavor/inert-flavor/tests/tests_inner/inert_flavor.rs";
                let line = 50usize;
                let col = 17usize;
                let expr = "enresult(())?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n",
                            path,
                            line,
                            col,
                            e,
                            expr,
                        ),
                    );
                };
            })?;
        Ok(())
    }
    fn method_override() -> Result<(), ()> {
        enresult(())
            .inspect_err(|e| {
                let path = "hooq-macros/tests/special/flavor/inert-flavor/tests/tests_inner/inert_flavor.rs";
                let line = 57usize;
                let col = 17usize;
                let expr = "enresult(())?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n",
                            path,
                            line,
                            col,
                            e,
                            expr,
                        ),
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
        enresult(())
            .inspect_err(|e| {
                let path = "hooq-macros/tests/special/flavor/inert-flavor/tests/tests_inner/inert_flavor.rs";
                let line = 74usize;
                let col = 17usize;
                let expr = "enresult(())?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n",
                            path,
                            line,
                            col,
                            e,
                            expr,
                        ),
                    );
                };
            })?;
        Ok(())
    }
    fn hook_targets_override() -> Result<(), ()> {
        let f = || {
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Err::<(), ()>(())
                    .inspect_err(|_| {
                        ::std::io::_print(format_args!("custom error handler called\n"));
                    });
            }
            enresult(())
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?;
            if enresult(true)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Ok(())
                    .inspect_err(|_| {
                        ::std::io::_print(format_args!("custom error handler called\n"));
                    });
            }
            Err::<(), ()>(())
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })
        };
        f()
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })?;
        let f = || {
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Err::<(), ()>(());
            }
            enresult(())
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?;
            if enresult(true)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Ok(());
            }
            Err::<(), ()>(())
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })
        };
        f()
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })?;
        Ok(())
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })
    }
    fn tail_expr_idents_override() -> Result<(), ()> {
        let f = || {
            enresult(())
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?;
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return MyOk();
            }
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return MyErr();
            }
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Err::<(), ()>(())
                    .inspect_err(|_| {
                        ::std::io::_print(format_args!("custom error handler called\n"));
                    });
            }
            if enresult(true)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Ok(())
                    .inspect_err(|_| {
                        ::std::io::_print(format_args!("custom error handler called\n"));
                    });
            }
            Ok(())
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })
        };
        f()
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })?;
        let f = || {
            enresult(())
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?;
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return MyOk();
            }
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return MyErr()
                    .inspect_err(|_| {
                        ::std::io::_print(format_args!("custom error handler called\n"));
                    });
            }
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Err::<(), ()>(());
            }
            if enresult(true)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return Ok(());
            }
            Ok(())
        };
        f()
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })?;
        Ok(())
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })
    }
    fn result_types_override() -> Result<(), ()> {
        let f = || -> Result<(), ()> {
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return enresult(())
                    .inspect_err(|_| {
                        ::std::io::_print(format_args!("custom error handler called\n"));
                    });
            }
            Ok(())
        };
        f()
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })?;
        let f = || -> Result<(), ()> {
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return enresult(());
            }
            Ok(())
        };
        f()
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })?;
        let f = || -> MyResult {
            if enresult(false)
                .inspect_err(|_| {
                    ::std::io::_print(format_args!("custom error handler called\n"));
                })?
            {
                return enresult(())
                    .inspect_err(|_| {
                        ::std::io::_print(format_args!("custom error handler called\n"));
                    });
            }
            Ok(())
        };
        f()
            .inspect_err(|_| {
                ::std::io::_print(format_args!("custom error handler called\n"));
            })?;
        Ok(())
    }
    fn hook_in_macros_override() -> Result<(), ()> {
        {
            ::std::io::_print(
                format_args!(
                    "{0}\n",
                    enresult(10)
                        .inspect_err(|e| {
                            let path = "hooq-macros/tests/special/flavor/inert-flavor/tests/tests_inner/inert_flavor.rs";
                            let line = 220usize;
                            let col = 32usize;
                            let expr = "enresult(10)?";
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "[{0}:{1}:{2}] {3:?}\n    {4}\n",
                                        path,
                                        line,
                                        col,
                                        e,
                                        expr,
                                    ),
                                );
                            };
                        })?,
                ),
            );
        };
        {
            ::std::io::_print(format_args!("{0}\n", enresult(10)?));
        };
        Ok(())
    }
    fn bindings_override() -> Result<(), ()> {
        enresult(())
            .inspect_err(|_| {
                ::std::io::_print(format_args!("{0}\n", "(not specified)"));
            })?;
        enresult(())
            .inspect_err(|_| {
                ::std::io::_print(format_args!("{0}\n", "Hello"));
            })?;
        Ok(())
    }
    extern crate test;
    #[rustc_test_marker = "tests_inner::inert_flavor::test"]
    #[doc(hidden)]
    pub const test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests_inner::inert_flavor::test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "hooq-macros/tests/special/flavor/inert-flavor/tests/tests_inner/inert_flavor.rs",
            start_line: 241usize,
            start_col: 4usize,
            end_line: 241usize,
            end_col: 8usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test())),
    };
    fn test() {
        flavor_override().unwrap();
        method_override().unwrap();
        hook_targets_override().unwrap();
        tail_expr_idents_override().unwrap();
        result_types_override().unwrap();
        hook_in_macros_override().unwrap();
        bindings_override().unwrap();
    }
}
