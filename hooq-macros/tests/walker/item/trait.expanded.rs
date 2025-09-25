use hooq_macros::hooq;
mod tmp {
    use util_macros::id;
    #[allow(unused)]
    fn hoge() -> Result<(), ()> {
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                };
            })
    }
    pub trait Trit {
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
        fn g() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "related function"),
                        );
                    };
                })?;
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "related function"),
                        );
                    };
                })
        }
        fn h() -> bool {
            true
        }
        #[allow(unused)]
        fn i(&self) -> Result<(), ()> {
            let res = Err(());
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "method"));
                    };
                })?;
            res.inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "method"));
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
            Err(())
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
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })
        }
    }
}
