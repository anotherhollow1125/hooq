use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/call.rs";
            let line = 5usize;
            let col = 5usize;
            let expr = "   5|     Err(())\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    (|_, _, _| {
        hoge()
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
    })(
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?,
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?,
            10,
        )
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
