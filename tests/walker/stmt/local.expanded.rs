use hooq::hooq;
fn func() -> Result<(), ()> {
    #[allow(clippy::redundant_closure_call)]
    let _ = (|| {
        Ok(true)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "inner"));
                };
            })
    })()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "outer"));
            };
        })?;
    let n = 1;
    let 1 = n else {
        return Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "else"));
                };
            });
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
