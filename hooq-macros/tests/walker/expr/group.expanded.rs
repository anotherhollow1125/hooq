use hooq_macros::hooq;
fn hoge() -> Result<u32, ()> {
    Ok(10)
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
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
