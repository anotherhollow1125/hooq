use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/block.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
#[allow(clippy::unit_arg)]
fn func(flag: bool) -> Result<(), ()> {
    {
        {
            ::std::io::_print(format_args!("beep\n"));
        };
        Ok(
                {
                    Ok({
                            {
                                if flag {
                                    return hoge()
                                        .inspect(|_| {
                                            {
                                                ::std::io::_print(format_args!("tag: {0}\n", "4"));
                                            };
                                        });
                                }
                                {
                                    if flag {
                                        return Err(())
                                            .inspect(|_| {
                                                {
                                                    ::std::io::_print(format_args!("tag: {0}\n", "5"));
                                                };
                                            });
                                    }
                                    Err(())
                                        .inspect(|_| {
                                            {
                                                ::std::io::_print(format_args!("tag: {0}\n", "5"));
                                            };
                                        })
                                }
                                    .inspect(|_| {
                                        {
                                            ::std::io::_print(format_args!("tag: {0}\n", "5"));
                                        };
                                    })?;
                                {
                                    {
                                        ::std::io::_print(format_args!("beepbeep\n"));
                                    };
                                }
                            }
                        })
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "3"));
                            };
                        })
                }
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "2"));
                        };
                    })?,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "2"));
                };
            })
    }
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "1"));
            };
        })
}
