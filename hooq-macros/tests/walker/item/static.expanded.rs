use hooq_macros::hooq;
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/item/static.rs";
            let line = 8usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    static _STATIC_VAR: u32 = {
        fn _f(flag: bool) -> Result<(), ()> {
            if flag {
                return Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                        };
                    });
            }
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                    };
                })?;
            let res = {
                if flag {
                    return hoge()
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "deep"));
                            };
                        });
                }
                Ok(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "deep"));
                        };
                    })
            };
            res.inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                };
            })
        }
        10
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
