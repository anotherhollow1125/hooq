use hooq_macros::hooq;
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
}
fn func() -> Result<(), ()> {
    let b = !enresult(true)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    let v = -enresult({
            if b {
                return Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    });
            }
            1
        })
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    let v = *enresult(&v)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })? + 1;
    let _ = !enresult({
            if v > 0 {
                return enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    });
            }
            enresult(false)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "nested"));
                    };
                })?
        })
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
