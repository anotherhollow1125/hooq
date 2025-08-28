use hooq_macros::hooq;
fn hoge() -> Result<u32, ()> {
    Ok(10)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/group.rs";
            let line = 9usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
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
