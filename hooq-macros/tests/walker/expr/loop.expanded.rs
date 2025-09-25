use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/loop.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn func(flags: Vec<bool>) -> Result<(), ()> {
    let mut flags = flags.into_iter();
    loop {
        hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
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
        if flags.next().unwrap_or(false) {
            return hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                });
        }
        if flags.next().unwrap_or(false) {
            break Err(());
        }
        if flags.next().unwrap_or(false) {
            break Ok(
                hoge()
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    })?,
            );
        }
        if flags.next().unwrap_or(false) {
            break hoge();
        }
    }
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
