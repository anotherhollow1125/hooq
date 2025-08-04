use hooq::hooq;
unsafe fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/unsafe.rs", 5usize
                    ),
                );
            };
        })
}
fn func(flags: Vec<bool>) -> Result<(), ()> {
    let mut flags = flags.into_iter();
    unsafe {
        hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "in unsafe"));
                };
            })?;
        if flags.next().unwrap_or(false) {
            return Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                });
        }
        let _ = || {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "in closure"));
                    };
                })?;
            hoge()
        };
        if flags.next().unwrap_or(false) {
            return hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                });
        }
    }
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
