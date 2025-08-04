use hooq::hooq;
fn hoge(v: usize) -> Result<usize, ()> {
    Ok(v + 1)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/let.rs", 5usize,
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    if let 11 = hoge(10)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?
        && let 12 = hoge(11)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "second let"));
                };
            })?
    {
        {
            ::std::io::_print(format_args!("hoge is 10\n"));
        };
        let _ = hoge(0)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                };
            })?;
        hoge(0).map(|_| ())
    } else {
        {
            ::std::io::_print(format_args!("hoge is not 10\n"));
        };
        let _ = hoge(0)
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
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
