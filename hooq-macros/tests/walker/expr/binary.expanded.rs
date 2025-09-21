use hooq_macros::hooq;
fn b() -> Result<u32, ()> {
    Ok(10)
}
fn c() -> Result<u32, ()> {
    Ok(20)
}
fn func() -> Result<u32, ()> {
    let mut a = b()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "add"));
            };
        })?
        + c()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "add"));
                };
            })?;
    {
        a
            += b()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "add_assign"));
                    };
                })?;
    }
    Ok(
        a
            + b()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?
            + c()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?,
    )
}
