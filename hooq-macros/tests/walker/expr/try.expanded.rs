use hooq_macros::hooq;
fn func() -> Result<(), ()> {
    #[allow(clippy::redundant_closure_call)]
    (|| {
        (|| {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "inner inner"));
                    };
                })
        })()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "inner"));
                };
            })?;
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "inner"));
                };
            })
    })()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "try"));
            };
        })?;
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
