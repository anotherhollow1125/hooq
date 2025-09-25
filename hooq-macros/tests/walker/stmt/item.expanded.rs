use hooq_macros::hooq;
fn func() -> Result<(), ()> {
    fn f() -> Result<(), ()> {
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "func"));
                };
            })
    }
    f()
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
