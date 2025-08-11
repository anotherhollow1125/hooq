use hooq::hooq;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/item/macro.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
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
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Ok(())"),
                );
            };
        })
}
