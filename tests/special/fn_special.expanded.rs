use hooq::hooq;
fn func(flag: bool, flog: bool) -> Result<(), ()> {
    if flag {
        return Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "return"));
                };
            });
    }
    fn hoge(flag: bool) -> u32 {
        if flag {
            return 1;
        }
        0
    }
    let _ = hoge(flag);
    fn fuga(flag: bool) -> u32 {
        let a = |b| {
            if b {
                Ok(10)
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                        };
                    })
            } else {
                Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                        };
                    })
            }
        };
        let b = || -> Result<u32, ()> {
            Ok(10)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                    };
                })
        };
        let c = || -> Result<u32, ()> {
            b()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                    };
                })
        };
        let d = || -> Result<u32, ()> {
            if flag {
                {
                    return c()
                        .inspect(|_| {
                            {
                                ::std::io::_print(
                                    format_args!("tag: {0}\n", "return in deep"),
                                );
                            };
                        });
                }
            }
            fn dd() -> u32 {
                10
            }
            let ddd = || {
                if flag {
                    return hoge(true);
                }
                let dddd = || {
                    if flag {
                        return b();
                    }
                    let ddddd = || {
                        let res = c()
                            .inspect(|_| {
                                {
                                    ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                                };
                            })? + 10;
                        Result::<u32, ()>::Ok(res)
                            .inspect(|_| {
                                {
                                    ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                                };
                            })
                    };
                    ddddd()
                };
                20 + dddd().unwrap()
            };
            Ok(dd() + ddd())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                    };
                })
        };
        let e = || {
            if flag {
                Ok(
                        a(true)
                            .inspect(|_| {
                                {
                                    ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                                };
                            })?
                            + b()
                                .inspect(|_| {
                                    {
                                        ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                                    };
                                })?
                            + c()
                                .inspect(|_| {
                                    {
                                        ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                                    };
                                })?
                            + d()
                                .inspect(|_| {
                                    {
                                        ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                                    };
                                })?,
                    )
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                        };
                    })
            } else {
                Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                        };
                    })
            }
        };
        let v = e().unwrap();
        v + 10
    }
    let _ = fuga(flog);
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
