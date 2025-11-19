use hooq_macros::hooq;
unsafe fn hoge() -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/unsafe.rs";
            let line = 5usize;
            let col = 5usize;
            let expr = "   5>    Err(())\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
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
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
