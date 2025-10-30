use hooq_macros::hooq;
use util_macros::empty;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}
fn func() -> Result<(), ()> {
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult(10).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n", "outer",
                "enresult(10)")); }; }) ?
            ),
        );
    };
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult(20).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n", "inner",
                "enresult(20)")); }; }) ?
            ),
        );
    };
    #[allow(unused)]
    mod tmp {
        use util_macros::id;
        use super::enresult;
        #[allow(unused)]
        macro_rules! tmp {
            () => {
                fn tmp_fn(flag : bool) -> Result < (), () > { if flag { enresult(()) ?;
                return Err(()); } Err(()) }
            };
        }
        #[allow(unused)]
        fn outer() -> Result<(), ()> {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "outer", "enresult(())"
                            ),
                        );
                    };
                })?;
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}, expr: {1}\n", "outer", "Err(())"),
                        );
                    };
                })
        }
        const _CONST_VAL: usize = {
            fn _f() -> Result<(), ()> {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "tag: {0}, expr: {1}\n", "inner func", "enresult(())"
                                ),
                            );
                        };
                    })?;
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "tag: {0}, expr: {1}\n", "inner func", "enresult(())"
                                ),
                            );
                        };
                    })
            }
            10
        };
        #[allow(unused)]
        fn inner() -> Result<(), ()> {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner", "enresult(())"
                            ),
                        );
                    };
                })?;
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Err(())"),
                        );
                    };
                })
        }
    }
    let _ = ::alloc::vec::from_elem(
        enresult(10)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "outer", "enresult(10)"),
                    );
                };
            })?,
        enresult(2)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "outer", "enresult(2)"),
                    );
                };
            })?,
    );
    let _ = <[_]>::into_vec(
        ::alloc::boxed::box_new([
            enresult(10)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner 1", "enresult(10)"
                            ),
                        );
                    };
                })?,
            enresult(20)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner 2", "enresult(20)"
                            ),
                        );
                    };
                })?,
            enresult(30)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner 3", "enresult(30)"
                            ),
                        );
                    };
                })?,
        ]),
    );
    macro_rules! stmts_with_print {
        ($($s:stmt, $e:expr);*) => {
            $($s println!("{}", $e);)*
        };
    }
    if enresult(true)
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!(
                        "tag: {0}, expr: {1}\n", "stmts_with_print", "enresult(true)"
                    ),
                );
            };
        })?
    {
        {
            ::std::io::_print(format_args!("It\'s true\n"));
        };
    }
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult("if let").inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n",
                "stmts_with_print", "enresult(\"if let\")")); }; }) ?
            ),
        );
    };
    for _ in enresult([1, 2])
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!(
                        "tag: {0}, expr: {1}\n", "stmts_with_print", "enresult([1, 2])"
                    ),
                );
            };
        })?
    {}
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult("for loop").inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n",
                "stmts_with_print", "enresult(\"for loop\")")); }; }) ?
            ),
        );
    };
    macro_rules! stmts_with_print_rev {
        ($($e:expr, $s:stmt);*) => {
            $($s println!("{}", $e);)*
        };
    }
    if enresult(true)
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!(
                        "tag: {0}, expr: {1}\n", "stmts_with_print", "enresult(true)"
                    ),
                );
            };
        })?
    {
        {
            ::std::io::_print(format_args!("It\'s true\n"));
        };
    }
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult("if let").inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n",
                "stmts_with_print", "enresult(\"if let\")")); }; }) ?
            ),
        );
    };
    for _ in enresult([1, 2])
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!(
                        "tag: {0}, expr: {1}\n", "stmts_with_print", "enresult([1, 2])"
                    ),
                );
            };
        })?
    {}
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult("for loop").inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n",
                "stmts_with_print", "enresult(\"for loop\")")); }; }) ?
            ),
        );
    };
    macro_rules! vecs {
        ($($v:expr; $n:expr),*) => {
            vec![$(vec![$v; $n]),*]
        };
    }
    let _ = <[_]>::into_vec(
        ::alloc::boxed::box_new([
            ::alloc::vec::from_elem(
                enresult(10)
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "tag: {0}, expr: {1}\n", "vecs", "enresult(10)"
                                ),
                            );
                        };
                    })?,
                enresult(2)
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}, expr: {1}\n", "vecs", "enresult(2)"),
                            );
                        };
                    })?,
            ),
            ::alloc::vec::from_elem(
                enresult(20)
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "tag: {0}, expr: {1}\n", "vecs", "enresult(20)"
                                ),
                            );
                        };
                    })?,
                enresult(3)
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}, expr: {1}\n", "vecs", "enresult(3)"),
                            );
                        };
                    })?,
            ),
        ]),
    );
    let _ = ::serde_json::Value::Object({
        let mut object = ::serde_json::Map::new();
        let _ = object.insert(("key").into(), ::serde_json::to_value(&"value").unwrap());
        let _ = object
            .insert(
                ("array").into(),
                ::serde_json::to_value(&enresult([1, 2, 3])?).unwrap(),
            );
        object
    });
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Err(())"),
                );
            };
        })
}
fn no_hooks_to_macros() -> Result<(), ()> {
    let _ = <[_]>::into_vec(
        ::alloc::boxed::box_new([enresult(1)?, enresult(2)?, enresult(3)?]),
    );
    enresult(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/macro.rs";
            let line = 172usize;
            let col = 17usize;
            let expr = "enresult(())?";
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                    ),
                );
            };
        })?;
    {
        ::std::io::_print(format_args!("{0}\n", enresult("beep") ?));
    };
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/macro.rs";
            let line = 176usize;
            let col = 5usize;
            let expr = "Err(())";
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                    ),
                );
            };
        })
}
