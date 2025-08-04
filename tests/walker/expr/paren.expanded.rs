use hooq::hooq;
fn hoge() -> Result<u32, ()> {
    Ok(10)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/paren.rs", 7usize
                    ),
                );
            };
        })
}
fn func(flag: bool) -> Result<(), ()> {
    let _ = (2
        * (3
            * hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?
            * (hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?) * 5
            * {
                if !flag {
                    return Err(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                            };
                        });
                }
                Result::<
                    u32,
                    (),
                >::Ok({
                        let tmp = hoge()
                            .inspect(|_| {
                                {
                                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                                };
                            })?;
                        if !flag {
                            return hoge()
                                .map(|_| ())
                                .inspect(|_| {
                                    {
                                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                                    };
                                });
                        }
                        hoge()
                            .inspect(|_| {
                                {
                                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                                };
                            })? + tmp
                    })
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    })
            }
                .unwrap())) * 6;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
