use hooq_macros::hooq;
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/item/impl.rs";
            let line = 6usize;
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
mod tmp {
    use util_macros::id;
    use super::hoge;
    pub struct S;
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
        pub fn g() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "impl related function"),
                        );
                    };
                })?;
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "impl related function"),
                        );
                    };
                })
        }
        pub fn h() -> bool {
            true
        }
        #[allow(unused)]
        fn i(&self) -> Result<(), ()> {
            let res = Err(());
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
        pub fn outer() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "outer"));
                    };
                })?;
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "outer"));
                    };
                })
        }
        pub fn inner() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "inner"));
                    };
                })?;
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "impl"));
                    };
                })
        }
    }
}
