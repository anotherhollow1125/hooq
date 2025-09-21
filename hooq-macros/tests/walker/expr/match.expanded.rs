use hooq_macros::hooq;
fn hoge() -> Result<usize, ()> {
    Ok(10)
}
fn func(flag: bool) -> Result<(), ()> {
    match hoge()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?
    {
        0..=10 => {
            {
                ::std::io::_print(format_args!("Matched 0..=10\n"));
            };
            hoge()
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
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })
        }
        _ => {
            {
                ::std::io::_print(format_args!("Matched other case\n"));
            };
            if !flag {
                return Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    });
            }
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })
        }
    }
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
