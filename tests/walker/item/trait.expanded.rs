use hooq::hooq;
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/item/trait.rs", 8usize,
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    trait Trit {
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
                            format_args!("tag: {0}\n", "related function"),
                        );
                    };
                })?;
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "related function"),
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
    }
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
