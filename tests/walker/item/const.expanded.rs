use hooq::hooq;
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/item/const.rs", 8usize
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    const _STATIC_VAR: u32 = {
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
