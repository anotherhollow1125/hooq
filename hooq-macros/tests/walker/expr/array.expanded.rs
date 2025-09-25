use hooq_macros::hooq;
fn hoge() -> Result<u32, [u32; 2]> {
    Ok(10)
}
fn func() -> Result<(), [u32; 2]> {
    Err([
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "first"));
                    };
                })?,
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "second"));
                    };
                })?,
        ])
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
