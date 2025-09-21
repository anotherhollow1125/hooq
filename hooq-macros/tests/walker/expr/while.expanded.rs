use hooq_macros::hooq;
fn get_bool(i: &mut usize) -> Result<bool, ()> {
    *i += 1;
    Ok(*i < 5)
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
            return Err(())
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
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
