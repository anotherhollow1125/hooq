use hooq_macros::hooq;
struct Hoge {
    field: u32,
}
fn hoge() -> Result<Hoge, ()> {
    Ok(Hoge { field: 10 })
}
fn func() -> Result<(), ()> {
    let _ = hoge()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?
        .field;
    let _ = {
        hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "inner"));
                };
            })?
    }
        .field;
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
