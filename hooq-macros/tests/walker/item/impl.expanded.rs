use hooq_macros::hooq;
use util_macros::id;
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/item/impl.rs";
            let line = 7usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    #[allow(unused)]
    struct S;
    impl S {
        const _CONST_VAL: usize = {
            fn _f() -> Result<(), ()> {
                hoge()
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                        };
                    })?;
                hoge()
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                        };
                    })
            }
            10
        };
        #[allow(unused)]
        fn g() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "impl related function"),
                        );
                    };
                })?;
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "impl related function"),
                        );
                    };
                })
        }
        #[allow(unused)]
        fn h() -> bool {
            true
        }
        #[allow(unused)]
        fn i(&self) -> Result<(), ()> {
            let res = Ok(());
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "impl method"));
                    };
                })?;
            res.inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "impl method"));
                };
            })
        }
        #[allow(unused)]
        fn j(&self) -> bool {
            true
        }
        #[allow(unused)]
        fn outer() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "outer"));
                    };
                })?;
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "outer"));
                    };
                })
        }
        #[allow(unused)]
        fn inner() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "inner"));
                    };
                })?;
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "impl"));
                    };
                })
        }
    }
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
