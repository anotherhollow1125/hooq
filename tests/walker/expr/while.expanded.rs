use hooq::hooq;
fn get_bool(i: &mut usize) -> Result<bool, ()> {
    *i += 1;
    Ok(*i < 5)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/while.rs";
            let line = 7usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let mut i = 0;
    while get_bool(&mut i)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "while"));
            };
        })?
    {
        let mut j = 0;
        let _ = get_bool(&mut j)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "while"));
                };
            })?;
        if !get_bool(&mut j)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "1"));
                };
            })?
        {
            get_bool(&mut j)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "1"));
                    };
                })?;
            return Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "1"));
                    };
                });
        }
        if !get_bool(&mut j)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "2"));
                };
            })?
        {
            get_bool(&mut j)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "2"));
                    };
                })?;
            return get_bool(&mut j)
                .map(|_| ())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "2"));
                    };
                });
        }
        if !get_bool(&mut j)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "3"));
                };
            })?
        {
            get_bool(&mut j)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "3"));
                    };
                })?;
            break;
        }
    }
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
