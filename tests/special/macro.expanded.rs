use hooq::hooq;
use util_macros::empty;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/macro.rs";
            let line = 6usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
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
                return Err(()); } Ok(()) }
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
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}, expr: {1}\n", "outer", "Ok(())"),
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
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Ok(())"),
                        );
                    };
                })
        }
    }
    let _ = <[_]>::into_vec(
        ::alloc::boxed::box_new([
            enresult(10)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "outer", "enresult(10)"
                            ),
                        );
                    };
                })?,
            20,
            enresult(30)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "outer", "enresult(30)"
                            ),
                        );
                    };
                })?,
        ]),
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
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Ok(())"),
                );
            };
        })
}
