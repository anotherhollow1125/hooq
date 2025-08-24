use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/call.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
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
        Ok(())
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
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
