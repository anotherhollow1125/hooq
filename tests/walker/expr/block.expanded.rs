use hooq::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/block.rs", 5usize,
                    ),
                );
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
                                    Ok(())
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
