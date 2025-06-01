use hooq::hooq;
fn func() -> Result<(), ()> {
    #[allow(clippy::redundant_closure_call)]
    (|| {
        (|| Ok(()))()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "inner"));
                };
            })?;
        Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "inner"));
                };
            })
    })()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "try"));
            };
        })?;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
