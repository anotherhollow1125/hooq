#![allow(unused_braces)]
use hooq::hooq;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/special/skip.rs", 7usize,
                    ),
                );
            };
        })
}
fn skip_target() -> Result<(), ()> {
    enresult(())?;
    if enresult(false)? {
        return Ok(());
    }
    if enresult(false)? {
        return enresult(());
    }
    let _: Result<(), ()> = {
        enresult(())?;
        if enresult(false)? {
            enresult(())?;
            return Ok(());
        }
        Ok(())
    };
    Ok(())
}
fn complex_skip() -> Result<(), ()> {
    let gen_bools = || enresult(true);
    if gen_bools()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "1"));
            };
        })?
    {
        if gen_bools()? {
            if gen_bools()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "2"));
                    };
                })?
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "3"));
                        };
                    })?;
                if enresult(false)? {
                    enresult(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "2"));
                            };
                        })?;
                    return { Ok(()) }
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "2"));
                            };
                        });
                }
                Ok(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "2"));
                        };
                    })
            } else {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "2"));
                        };
                    })?;
                if enresult(false)? {
                    enresult(())?;
                    return Err(());
                }
                if enresult(false)
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "2"));
                        };
                    })?
                {
                    if enresult(false)
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "2"));
                            };
                        })?
                    {
                        return {
                            Err(())
                                .inspect(|_| {
                                    {
                                        ::std::io::_print(format_args!("tag: {0}\n", "2"));
                                    };
                                })
                        }
                            .inspect(|_| {
                                {
                                    ::std::io::_print(format_args!("tag: {0}\n", "2"));
                                };
                            });
                    }
                    return {
                        {
                            Err(())
                                .inspect(|_| {
                                    {
                                        ::std::io::_print(format_args!("tag: {0}\n", "2"));
                                    };
                                })
                        }
                    };
                }
                Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "2"));
                        };
                    })
            }
        } else {
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "2"));
                    };
                })
        }
    } else {
        Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "1"));
                };
            })
    }
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "1"));
            };
        })
}
fn func() -> Result<(), ()> {
    skip_target()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    complex_skip()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
